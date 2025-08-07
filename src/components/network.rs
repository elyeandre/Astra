use mlua::{Lua, Result, UserData, UserDataMethods, Value, Integer};
use rtnetlink::{Handle, new_connection};
use netlink_packet_route::{
    link::{LinkMessage, LinkFlags, LinkAttribute},
    route::{RouteMessage, RouteAttribute, RouteType, RouteScope, RouteProtocol, RouteAddress},
    AddressFamily,
};
use std::collections::HashMap;
use std::net::IpAddr;
use std::str::FromStr;
use tokio::time::{timeout, Duration};
use futures::stream::TryStreamExt;

/// Network interface management component
pub struct NetworkManager {
    handle: Handle,
}

impl NetworkManager {
    /// Create a new NetworkManager instance
    pub async fn new() -> Result<Self> {
        let (connection, handle, _) = new_connection()
            .map_err(|e| mlua::Error::runtime(format!("Failed to create netlink connection: {}", e)))?;

        // Spawn the connection in the background
        tokio::spawn(connection);

        Ok(Self { handle })
    }

    /// Set a network interface up
    pub async fn set_link_up(&self, interface_name: String) -> Result<()> {
        tracing::debug!("Bringing interface '{}' up", interface_name);

        // Get the interface by name
        let mut links = self.handle.link().get().match_name(interface_name.clone()).execute();
        let link = timeout(Duration::from_secs(5), links.try_next())
            .await
            .map_err(|_| mlua::Error::runtime("Timeout getting interface information"))?
            .map_err(|e| mlua::Error::runtime(format!("Failed to get interface: {}", e)))?
            .ok_or_else(|| mlua::Error::runtime(format!("Interface '{}' not found", interface_name)))?;

        // Create a modified link message with UP flag set
        let mut new_link = LinkMessage::default();
        new_link.header.index = link.header.index;
        new_link.header.flags = link.header.flags | LinkFlags::Up;
        
        // Set the interface up
        let result = timeout(
            Duration::from_secs(10),
            self.handle.link().set(new_link).execute()
        ).await;

        match result {
            Ok(Ok(_)) => {
                tracing::info!("Successfully brought interface '{}' up", interface_name);
                Ok(())
            },
            Ok(Err(e)) => {
                tracing::error!("Failed to bring interface '{}' up: {}", interface_name, e);
                Err(mlua::Error::runtime(format!("Failed to bring interface up: {}", e)))
            },
            Err(_) => {
                tracing::error!("Timeout bringing interface '{}' up", interface_name);
                Err(mlua::Error::runtime("Timeout bringing interface up"))
            }
        }
    }

    /// Set a network interface down
    pub async fn set_link_down(&self, interface_name: String) -> Result<()> {
        tracing::debug!("Bringing interface '{}' down", interface_name);

        // Get the interface by name
        let mut links = self.handle.link().get().match_name(interface_name.clone()).execute();
        let link = timeout(Duration::from_secs(5), links.try_next())
            .await
            .map_err(|_| mlua::Error::runtime("Timeout getting interface information"))?
            .map_err(|e| mlua::Error::runtime(format!("Failed to get interface: {}", e)))?
            .ok_or_else(|| mlua::Error::runtime(format!("Interface '{}' not found", interface_name)))?;

        // Create a modified link message with UP flag cleared
        let mut new_link = LinkMessage::default();
        new_link.header.index = link.header.index;
        new_link.header.flags = link.header.flags & !LinkFlags::Up;
        
        // Set the interface down
        let result = timeout(
            Duration::from_secs(10),
            self.handle.link().set(new_link).execute()
        ).await;

        match result {
            Ok(Ok(_)) => {
                tracing::info!("Successfully brought interface '{}' down", interface_name);
                Ok(())
            },
            Ok(Err(e)) => {
                tracing::error!("Failed to bring interface '{}' down: {}", interface_name, e);
                Err(mlua::Error::runtime(format!("Failed to bring interface down: {}", e)))
            },
            Err(_) => {
                tracing::error!("Timeout bringing interface '{}' down", interface_name);
                Err(mlua::Error::runtime("Timeout bringing interface down"))
            }
        }
    }

    /// Set MTU for a network interface
    pub async fn set_link_mtu(&self, interface_name: String, mtu: u32) -> Result<()> {
        tracing::debug!("Setting MTU for interface '{}' to {}", interface_name, mtu);

        // Validate MTU range
        if mtu < 68 || mtu > 65536 {
            return Err(mlua::Error::runtime(format!("Invalid MTU value: {}. MTU must be between 68 and 65536", mtu)));
        }

        // Get the interface by name
        let mut links = self.handle.link().get().match_name(interface_name.clone()).execute();
        let link = timeout(Duration::from_secs(5), links.try_next())
            .await
            .map_err(|_| mlua::Error::runtime("Timeout getting interface information"))?
            .map_err(|e| mlua::Error::runtime(format!("Failed to get interface: {}", e)))?
            .ok_or_else(|| mlua::Error::runtime(format!("Interface '{}' not found", interface_name)))?;

        // Create a modified link message with MTU attribute
        let mut new_link = LinkMessage::default();
        new_link.header.index = link.header.index;
        new_link.attributes.push(LinkAttribute::Mtu(mtu));
        
        // Set the MTU
        let result = timeout(
            Duration::from_secs(10),
            self.handle.link().set(new_link).execute()
        ).await;

        match result {
            Ok(Ok(_)) => {
                tracing::info!("Successfully set MTU for interface '{}' to {}", interface_name, mtu);
                Ok(())
            },
            Ok(Err(e)) => {
                tracing::error!("Failed to set MTU for interface '{}': {}", interface_name, e);
                Err(mlua::Error::runtime(format!("Failed to set MTU: {}", e)))
            },
            Err(_) => {
                tracing::error!("Timeout setting MTU for interface '{}'", interface_name);
                Err(mlua::Error::runtime("Timeout setting MTU"))
            }
        }
    }

    /// Add a route
    pub async fn add_route(&self, route_spec: String) -> Result<()> {
        tracing::debug!("Adding route: {}", route_spec);
        
        let route_msg = self.parse_route_spec(&route_spec, true).await?;
        
        let result = timeout(
            Duration::from_secs(10),
            self.handle.route().add(route_msg).execute()
        ).await;

        match result {
            Ok(Ok(_)) => {
                tracing::info!("Successfully added route: {}", route_spec);
                Ok(())
            },
            Ok(Err(e)) => {
                tracing::error!("Failed to add route '{}': {}", route_spec, e);
                Err(mlua::Error::runtime(format!("Failed to add route: {}", e)))
            },
            Err(_) => {
                tracing::error!("Timeout adding route: {}", route_spec);
                Err(mlua::Error::runtime("Timeout adding route"))
            }
        }
    }

    /// Delete a route
    pub async fn delete_route(&self, route_spec: String) -> Result<()> {
        tracing::debug!("Deleting route: {}", route_spec);
        
        let route_msg = self.parse_route_spec(&route_spec, false).await?;
        
        let result = timeout(
            Duration::from_secs(10),
            self.handle.route().del(route_msg).execute()
        ).await;

        match result {
            Ok(Ok(_)) => {
                tracing::info!("Successfully deleted route: {}", route_spec);
                Ok(())
            },
            Ok(Err(e)) => {
                tracing::error!("Failed to delete route '{}': {}", route_spec, e);
                Err(mlua::Error::runtime(format!("Failed to delete route: {}", e)))
            },
            Err(_) => {
                tracing::error!("Timeout deleting route: {}", route_spec);
                Err(mlua::Error::runtime("Timeout deleting route"))
            }
        }
    }

    /// Parse route specification string into RouteMessage
    async fn parse_route_spec(&self, route_spec: &str, is_add: bool) -> Result<RouteMessage> {
        let parts: Vec<&str> = route_spec.split_whitespace().collect();
        if parts.is_empty() {
            return Err(mlua::Error::runtime("Empty route specification"));
        }

        let mut route_msg = RouteMessage::default();
        let mut i = 0;

        // Determine address family and destination
        let (destination, prefix_len, family) = if parts[i] == "default" {
            i += 1;
            (None, 0, AddressFamily::Inet) // Default to IPv4, will be corrected if IPv6 gateway
        } else {
            // Parse destination network
            let dest_parts: Vec<&str> = parts[i].split('/').collect();
            let dest_addr = IpAddr::from_str(dest_parts[0])
                .map_err(|_| mlua::Error::runtime(format!("Invalid destination address: {}", dest_parts[0])))?;
            
            let prefix_len = if dest_parts.len() > 1 {
                dest_parts[1].parse::<u8>()
                    .map_err(|_| mlua::Error::runtime(format!("Invalid prefix length: {}", dest_parts[1])))?
            } else {
                match dest_addr {
                    IpAddr::V4(_) => 32,
                    IpAddr::V6(_) => 128,
                }
            };

            let family = match dest_addr {
                IpAddr::V4(_) => AddressFamily::Inet,
                IpAddr::V6(_) => AddressFamily::Inet6,
            };

            i += 1;
            (Some(dest_addr), prefix_len, family)
        };

        // Set up route header
        route_msg.header.address_family = family;
        route_msg.header.destination_prefix_length = prefix_len;
        route_msg.header.table = 254; // RT_TABLE_MAIN
        route_msg.header.protocol = RouteProtocol::Boot;
        route_msg.header.scope = RouteScope::Universe;
        route_msg.header.kind = RouteType::Unicast;

        // Add destination if not default route
        if let Some(dest) = destination {
            match dest {
                IpAddr::V4(addr) => {
                    route_msg.attributes.push(RouteAttribute::Destination(RouteAddress::Inet(addr)));
                },
                IpAddr::V6(addr) => {
                    route_msg.attributes.push(RouteAttribute::Destination(RouteAddress::Inet6(addr)));
                }
            }
        }

        // Parse remaining parts
        while i < parts.len() {
            match parts[i] {
                "via" => {
                    i += 1;
                    if i >= parts.len() {
                        return Err(mlua::Error::runtime("Missing gateway address after 'via'"));
                    }
                    let gateway = IpAddr::from_str(parts[i])
                        .map_err(|_| mlua::Error::runtime(format!("Invalid gateway address: {}", parts[i])))?;
                    
                    // Update family if it was default and we have a gateway
                    if destination.is_none() {
                        route_msg.header.address_family = match gateway {
                            IpAddr::V4(_) => AddressFamily::Inet,
                            IpAddr::V6(_) => AddressFamily::Inet6,
                        };
                    }

                    match gateway {
                        IpAddr::V4(addr) => {
                            route_msg.attributes.push(RouteAttribute::Gateway(RouteAddress::Inet(addr)));
                        },
                        IpAddr::V6(addr) => {
                            route_msg.attributes.push(RouteAttribute::Gateway(RouteAddress::Inet6(addr)));
                        }
                    }
                },
                "dev" => {
                    i += 1;
                    if i >= parts.len() {
                        return Err(mlua::Error::runtime("Missing interface name after 'dev'"));
                    }
                    let interface_name = parts[i];
                    
                    // Get interface index
                    let mut links = self.handle.link().get().match_name(interface_name.to_string()).execute();
                    let link = timeout(Duration::from_secs(5), links.try_next())
                        .await
                        .map_err(|_| mlua::Error::runtime("Timeout getting interface information"))?
                        .map_err(|e| mlua::Error::runtime(format!("Failed to get interface: {}", e)))?
                        .ok_or_else(|| mlua::Error::runtime(format!("Interface '{}' not found", interface_name)))?;

                    route_msg.attributes.push(RouteAttribute::Oif(link.header.index));
                },
                "metric" => {
                    i += 1;
                    if i >= parts.len() {
                        return Err(mlua::Error::runtime("Missing metric value after 'metric'"));
                    }
                    let metric: u32 = parts[i].parse()
                        .map_err(|_| mlua::Error::runtime(format!("Invalid metric value: {}", parts[i])))?;
                    route_msg.attributes.push(RouteAttribute::Priority(metric));
                },
                _ => {
                    return Err(mlua::Error::runtime(format!("Unknown route parameter: {}", parts[i])));
                }
            }
            i += 1;
        }

        Ok(route_msg)
    }

    /// Get interface status
    pub async fn get_link_status(&self, lua: &Lua, interface_name: String) -> Result<HashMap<String, Value>> {
        tracing::debug!("Getting status for interface '{}'", interface_name);

        let mut links = self.handle.link().get().match_name(interface_name.clone()).execute();
        let link = timeout(Duration::from_secs(5), links.try_next())
            .await
            .map_err(|_| mlua::Error::runtime("Timeout getting interface information"))?
            .map_err(|e| mlua::Error::runtime(format!("Failed to get interface: {}", e)))?
            .ok_or_else(|| mlua::Error::runtime(format!("Interface '{}' not found", interface_name)))?;

        let mut status = HashMap::new();
        status.insert("name".to_string(), Value::String(lua.create_string(interface_name)?));
        status.insert("index".to_string(), Value::Integer(link.header.index as Integer));
        
        // Check if interface is up
        let is_up = link.header.flags & LinkFlags::Up == LinkFlags::Up;
        status.insert("is_up".to_string(), Value::Boolean(is_up));
        
        // Check if interface is running
        let is_running = link.header.flags & LinkFlags::Running == LinkFlags::Running;
        status.insert("is_running".to_string(), Value::Boolean(is_running));

        // Get MTU if available
        for attr in &link.attributes {
            if let LinkAttribute::Mtu(mtu) = attr {
                status.insert("mtu".to_string(), Value::Integer(*mtu as Integer));
                break;
            }
        }

        Ok(status)
    }

    /// List all network interfaces
    pub async fn list_interfaces(&self, lua: &Lua) -> Result<Vec<HashMap<String, Value>>> {
        tracing::debug!("Listing all network interfaces");

        let mut links = self.handle.link().get().execute();
        let mut interfaces = Vec::new();

        while let Some(link) = timeout(Duration::from_secs(5), links.try_next())
            .await
            .map_err(|_| mlua::Error::runtime("Timeout listing interfaces"))?
            .map_err(|e| mlua::Error::runtime(format!("Failed to list interfaces: {}", e)))?
        {
            let mut interface_info = HashMap::new();
            
            // Get interface name
            if let Some(name_attr) = link.attributes.iter().find_map(|attr| {
                if let LinkAttribute::IfName(name) = attr {
                    Some(name.clone())
                } else {
                    None
                }
            }) {
                interface_info.insert("name".to_string(), Value::String(lua.create_string(name_attr)?));
            }

            interface_info.insert("index".to_string(), Value::Integer(link.header.index as Integer));
            
            let is_up = link.header.flags & LinkFlags::Up == LinkFlags::Up;
            interface_info.insert("is_up".to_string(), Value::Boolean(is_up));
            
            let is_running = link.header.flags & LinkFlags::Running == LinkFlags::Running;
            interface_info.insert("is_running".to_string(), Value::Boolean(is_running));

            // Get MTU if available
            for attr in &link.attributes {
                if let LinkAttribute::Mtu(mtu) = attr {
                    interface_info.insert("mtu".to_string(), Value::Integer(*mtu as Integer));
                    break;
                }
            }

            interfaces.push(interface_info);
        }

        Ok(interfaces)
    }
}

impl UserData for NetworkManager {
    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        methods.add_async_method("set_link_up", |_lua, this, interface_name: String| async move {
            this.set_link_up(interface_name).await
        });

        methods.add_async_method("set_link_down", |_lua, this, interface_name: String| async move {
            this.set_link_down(interface_name).await
        });

        methods.add_async_method("set_link_mtu", |_lua, this, (interface_name, mtu): (String, u32)| async move {
            this.set_link_mtu(interface_name, mtu).await
        });

        methods.add_async_method("add_route", |_lua, this, route_spec: String| async move {
            this.add_route(route_spec).await
        });

        methods.add_async_method("delete_route", |_lua, this, route_spec: String| async move {
            this.delete_route(route_spec).await
        });

        methods.add_async_method("get_link_status", |lua, this, interface_name: String| async move {
            let status = this.get_link_status(&lua, interface_name).await?;
            let table = lua.create_table()?;
            for (key, value) in status {
                table.set(key, value)?;
            }
            Ok(table)
        });

        methods.add_async_method("list_interfaces", |lua, this, _: ()| async move {
            let interfaces = this.list_interfaces(&lua).await?;
            let result = lua.create_table()?;
            
            for (i, interface) in interfaces.iter().enumerate() {
                let interface_table = lua.create_table()?;
                for (key, value) in interface {
                    interface_table.set(key.clone(), value.clone())?;
                }
                result.set(i + 1, interface_table)?;
            }
            
            Ok(result)
        });
    }
}

pub struct NetworkComponent;

impl NetworkComponent {
    pub async fn register_to_lua(lua: &Lua) -> mlua::Result<()> {
        tracing::debug!("Registering Network component to Lua");

        // Get or create Astra table
        let astra_table: mlua::Table = match lua.globals().get("Astra") {
            Ok(table) => table,
            Err(_) => {
                let table = lua.create_table()?;
                lua.globals().set("Astra", table.clone())?;
                table
            }
        };

        // Create Network constructor function
        let network_constructor = lua.create_async_function(|lua, _: ()| async move {
            tracing::debug!("Creating new NetworkManager instance");
            let manager = NetworkManager::new().await?;
            lua.create_userdata(manager)
        })?;

        // Create network subtable
        let network_table = lua.create_table()?;
        network_table.set("new", network_constructor)?;
        
        astra_table.set("Network", network_table)?;
        
        tracing::info!("Network component registered successfully");
        Ok(())
    }

    pub fn lua_code() -> &'static str {
        include_str!("network.lua")
    }
}
