use mlua::{Lua, Result, UserData, UserDataMethods, Value, Integer, Table};
use rtnetlink::{Handle, new_connection};
use netlink_packet_route::{
    link::{LinkMessage, LinkFlags, LinkAttribute},
    route::{RouteMessage, RouteHeader, RouteAttribute, RouteScope, RouteType, RouteProtocol},
    AddressFamily,
};
use std::collections::HashMap;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
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

            interfaces.push(interface_info);
        }

        Ok(interfaces)
    }

    /// Helper function to get interface index by name
    async fn get_interface_index(&self, interface_name: &str) -> Result<u32> {
        let mut links = self.handle.link().get().match_name(interface_name.to_string()).execute();
        let link = timeout(Duration::from_secs(5), links.try_next())
            .await
            .map_err(|_| mlua::Error::runtime("Timeout getting interface information"))?
            .map_err(|e| mlua::Error::runtime(format!("Failed to get interface: {}", e)))?
            .ok_or_else(|| mlua::Error::runtime(format!("Interface '{}' not found", interface_name)))?;
        
        Ok(link.header.index)
    }

    /// Parse IP address and prefix length from CIDR notation
    fn parse_cidr(destination: &str) -> Result<(IpAddr, u8)> {
        if let Some((ip_str, prefix_str)) = destination.split_once('/') {
            let ip: IpAddr = ip_str.parse()
                .map_err(|_| mlua::Error::runtime(format!("Invalid IP address: {}", ip_str)))?;
            let prefix: u8 = prefix_str.parse()
                .map_err(|_| mlua::Error::runtime(format!("Invalid prefix length: {}", prefix_str)))?;
            Ok((ip, prefix))
        } else {
            // If no prefix is provided, assume /32 for IPv4 or /128 for IPv6
            let ip: IpAddr = destination.parse()
                .map_err(|_| mlua::Error::runtime(format!("Invalid IP address: {}", destination)))?;
            let prefix = match ip {
                IpAddr::V4(_) => 32,
                IpAddr::V6(_) => 128,
            };
            Ok((ip, prefix))
        }
    }

    /// Add a route to the routing table
    pub async fn add_route(&self, route_table: Table) -> Result<()> {
        let destination: String = route_table.get("destination")?;
        let device: Option<String> = route_table.get("device").ok();
        let gateway: Option<String> = route_table.get("gateway").ok();

        tracing::debug!("Adding route: dest={}, device={:?}, gateway={:?}", 
                       destination, device, gateway);

        // Parse destination CIDR
        let (dest_ip, prefix_len) = Self::parse_cidr(&destination)?;

        // Create route message
        let mut route = RouteMessage::default();
        
        // Set address family
        route.header.address_family = match dest_ip {
            IpAddr::V4(_) => AddressFamily::Inet,
            IpAddr::V6(_) => AddressFamily::Inet6,
        };
        
        route.header.destination_prefix_length = prefix_len;
        route.header.scope = RouteScope::Universe;
        route.header.kind = RouteType::Unicast;
        route.header.protocol = RouteProtocol::Static;
        route.header.table = netlink_packet_route::RT_TABLE_MAIN;

        // Add destination attribute
        match dest_ip {
            IpAddr::V4(ipv4) => {
                if prefix_len > 0 {
                    route.attributes.push(RouteAttribute::Destination(ipv4.octets().to_vec()));
                }
            }
            IpAddr::V6(ipv6) => {
                if prefix_len > 0 {
                    route.attributes.push(RouteAttribute::Destination(ipv6.octets().to_vec()));
                }
            }
        }

        // Add gateway if provided
        if let Some(gw_str) = gateway {
            let gw_ip: IpAddr = gw_str.parse()
                .map_err(|_| mlua::Error::runtime(format!("Invalid gateway IP: {}", gw_str)))?;
            
            match gw_ip {
                IpAddr::V4(ipv4) => {
                    route.attributes.push(RouteAttribute::Gateway(ipv4.octets().to_vec()));
                }
                IpAddr::V6(ipv6) => {
                    route.attributes.push(RouteAttribute::Gateway(ipv6.octets().to_vec()));
                }
            }
        }

        // Add output interface if provided
        if let Some(dev_name) = device {
            let if_index = self.get_interface_index(&dev_name).await?;
            route.attributes.push(RouteAttribute::Oif(if_index));
        }

        // Execute the route addition
        let result = timeout(
            Duration::from_secs(10),
            self.handle.route().add(route).execute()
        ).await;

        match result {
            Ok(Ok(_)) => {
                tracing::info!("Successfully added route: {}", destination);
                Ok(())
            },
            Ok(Err(e)) => {
                tracing::error!("Failed to add route {}: {}", destination, e);
                Err(mlua::Error::runtime(format!("Failed to add route: {}", e)))
            },
            Err(_) => {
                tracing::error!("Timeout adding route: {}", destination);
                Err(mlua::Error::runtime("Timeout adding route"))
            }
        }
    }

    /// Delete a route from the routing table
    pub async fn delete_route(&self, route_table: Table) -> Result<()> {
        let destination: String = route_table.get("destination")?;
        let device: Option<String> = route_table.get("device").ok();
        let gateway: Option<String> = route_table.get("gateway").ok();

        tracing::debug!("Deleting route: dest={}, device={:?}, gateway={:?}", 
                       destination, device, gateway);

        // Parse destination CIDR
        let (dest_ip, prefix_len) = Self::parse_cidr(&destination)?;

        // Create route message for deletion
        let mut route = RouteMessage::default();
        
        // Set address family
        route.header.address_family = match dest_ip {
            IpAddr::V4(_) => AddressFamily::Inet,
            IpAddr::V6(_) => AddressFamily::Inet6,
        };
        
        route.header.destination_prefix_length = prefix_len;
        route.header.scope = RouteScope::Universe;
        route.header.kind = RouteType::Unicast;
        route.header.protocol = RouteProtocol::Static;
        route.header.table = netlink_packet_route::RT_TABLE_MAIN;

        // Add destination attribute
        match dest_ip {
            IpAddr::V4(ipv4) => {
                if prefix_len > 0 {
                    route.attributes.push(RouteAttribute::Destination(ipv4.octets().to_vec()));
                }
            }
            IpAddr::V6(ipv6) => {
                if prefix_len > 0 {
                    route.attributes.push(RouteAttribute::Destination(ipv6.octets().to_vec()));
                }
            }
        }

        // Add gateway if provided
        if let Some(gw_str) = gateway {
            let gw_ip: IpAddr = gw_str.parse()
                .map_err(|_| mlua::Error::runtime(format!("Invalid gateway IP: {}", gw_str)))?;
            
            match gw_ip {
                IpAddr::V4(ipv4) => {
                    route.attributes.push(RouteAttribute::Gateway(ipv4.octets().to_vec()));
                }
                IpAddr::V6(ipv6) => {
                    route.attributes.push(RouteAttribute::Gateway(ipv6.octets().to_vec()));
                }
            }
        }

        // Add output interface if provided
        if let Some(dev_name) = device {
            let if_index = self.get_interface_index(&dev_name).await?;
            route.attributes.push(RouteAttribute::Oif(if_index));
        }

        // Execute the route deletion
        let result = timeout(
            Duration::from_secs(10),
            self.handle.route().del(route).execute()
        ).await;

        match result {
            Ok(Ok(_)) => {
                tracing::info!("Successfully deleted route: {}", destination);
                Ok(())
            },
            Ok(Err(e)) => {
                tracing::error!("Failed to delete route {}: {}", destination, e);
                Err(mlua::Error::runtime(format!("Failed to delete route: {}", e)))
            },
            Err(_) => {
                tracing::error!("Timeout deleting route: {}", destination);
                Err(mlua::Error::runtime("Timeout deleting route"))
            }
        }
    }

    /// List routes in the routing table
    pub async fn list_routes(&self, lua: &Lua) -> Result<Vec<HashMap<String, Value>>> {
        tracing::debug!("Listing routing table");

        let mut routes = self.handle.route().get(AddressFamily::Unspec).execute();
        let mut route_list = Vec::new();

        while let Some(route_msg) = timeout(Duration::from_secs(10), routes.try_next())
            .await
            .map_err(|_| mlua::Error::runtime("Timeout listing routes"))?
            .map_err(|e| mlua::Error::runtime(format!("Failed to list routes: {}", e)))?
        {
            let mut route_info = HashMap::new();

            // Get destination
            let mut destination = "default".to_string();
            if let Some(RouteAttribute::Destination(dest_bytes)) = route_msg.attributes.iter()
                .find(|attr| matches!(attr, RouteAttribute::Destination(_)))
            {
                match route_msg.header.address_family {
                    AddressFamily::Inet if dest_bytes.len() == 4 => {
                        let ip = Ipv4Addr::new(dest_bytes[0], dest_bytes[1], dest_bytes[2], dest_bytes[3]);
                        destination = format!("{}/{}", ip, route_msg.header.destination_prefix_length);
                    }
                    AddressFamily::Inet6 if dest_bytes.len() == 16 => {
                        let mut octets = [0u8; 16];
                        octets.copy_from_slice(dest_bytes);
                        let ip = Ipv6Addr::from(octets);
                        destination = format!("{}/{}", ip, route_msg.header.destination_prefix_length);
                    }
                    _ => {}
                }
            } else if route_msg.header.destination_prefix_length == 0 {
                match route_msg.header.address_family {
                    AddressFamily::Inet => destination = "0.0.0.0/0".to_string(),
                    AddressFamily::Inet6 => destination = "::/0".to_string(),
                    _ => {}
                }
            }

            route_info.insert("destination".to_string(), Value::String(lua.create_string(destination)?));

            // Get gateway
            if let Some(RouteAttribute::Gateway(gw_bytes)) = route_msg.attributes.iter()
                .find(|attr| matches!(attr, RouteAttribute::Gateway(_)))
            {
                let gateway = match route_msg.header.address_family {
                    AddressFamily::Inet if gw_bytes.len() == 4 => {
                        let ip = Ipv4Addr::new(gw_bytes[0], gw_bytes[1], gw_bytes[2], gw_bytes[3]);
                        ip.to_string()
                    }
                    AddressFamily::Inet6 if gw_bytes.len() == 16 => {
                        let mut octets = [0u8; 16];
                        octets.copy_from_slice(gw_bytes);
                        let ip = Ipv6Addr::from(octets);
                        ip.to_string()
                    }
                    _ => "unknown".to_string(),
                };
                route_info.insert("gateway".to_string(), Value::String(lua.create_string(gateway)?));
            }

            // Get output interface
            if let Some(RouteAttribute::Oif(if_index)) = route_msg.attributes.iter()
                .find(|attr| matches!(attr, RouteAttribute::Oif(_)))
            {
                route_info.insert("if_index".to_string(), Value::Integer(*if_index as Integer));
            }

            route_info.insert("table".to_string(), Value::Integer(route_msg.header.table as Integer));
            route_info.insert("protocol".to_string(), Value::Integer(route_msg.header.protocol as u8 as Integer));

            route_list.push(route_info);
        }

        Ok(route_list)
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

        methods.add_async_method("add_route", |_lua, this, route_table: Table| async move {
            this.add_route(route_table).await
        });

        methods.add_async_method("delete_route", |_lua, this, route_table: Table| async move {
            this.delete_route(route_table).await
        });

        methods.add_async_method("list_routes", |lua, this, _: ()| async move {
            let routes = this.list_routes(&lua).await?;
            let result = lua.create_table()?;
            
            for (i, route) in routes.iter().enumerate() {
                let route_table = lua.create_table()?;
                for (key, value) in route {
                    route_table.set(key.clone(), value.clone())?;
                }
                result.set(i + 1, route_table)?;
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
