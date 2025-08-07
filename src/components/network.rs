use mlua::{Lua, Result, UserData, UserDataMethods, Value};
use rtnetlink::{Handle, new_connection};
use netlink_packet_route::link::LinkFlags;
use std::collections::HashMap;
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

        // Set the interface up by adding the UP flag
        let result = timeout(
            Duration::from_secs(10),
            self.handle
                .link()
                .set(link.header.index)
                .setattr_flags(LinkFlags::Up, LinkFlags::Up)
                .execute()
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

        // Set the interface down by removing the UP flag
        let result = timeout(
            Duration::from_secs(10),
            self.handle
                .link()
                .set(link.header.index)
                .unsetattr_flags(LinkFlags::Up)
                .execute()
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
    pub async fn get_link_status(&self, interface_name: String) -> Result<HashMap<String, Value>> {
        tracing::debug!("Getting status for interface '{}'", interface_name);

        let mut links = self.handle.link().get().match_name(interface_name.clone()).execute();
        let link = timeout(Duration::from_secs(5), links.try_next())
            .await
            .map_err(|_| mlua::Error::runtime("Timeout getting interface information"))?
            .map_err(|e| mlua::Error::runtime(format!("Failed to get interface: {}", e)))?
            .ok_or_else(|| mlua::Error::runtime(format!("Interface '{}' not found", interface_name)))?;

        let mut status = HashMap::new();
        status.insert("name".to_string(), Value::String(mlua::String::from(interface_name)));
        status.insert("index".to_string(), Value::Integer(link.header.index as i64));
        
        // Check if interface is up
        let is_up = link.header.flags & LinkFlags::Up == LinkFlags::Up;
        status.insert("is_up".to_string(), Value::Boolean(is_up));
        
        // Check if interface is running
        let is_running = link.header.flags & LinkFlags::Running == LinkFlags::Running;
        status.insert("is_running".to_string(), Value::Boolean(is_running));

        Ok(status)
    }

    /// List all network interfaces
    pub async fn list_interfaces(&self) -> Result<Vec<HashMap<String, Value>>> {
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
                if let netlink_packet_route::link::LinkAttribute::IfName(name) = attr {
                    Some(name.clone())
                } else {
                    None
                }
            }) {
                interface_info.insert("name".to_string(), Value::String(mlua::String::from(name_attr)));
            }

            interface_info.insert("index".to_string(), Value::Integer(link.header.index as i64));
            
            let is_up = link.header.flags & LinkFlags::Up == LinkFlags::Up;
            interface_info.insert("is_up".to_string(), Value::Boolean(is_up));
            
            let is_running = link.header.flags & LinkFlags::Running == LinkFlags::Running;
            interface_info.insert("is_running".to_string(), Value::Boolean(is_running));

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

        methods.add_async_method("get_link_status", |lua, this, interface_name: String| async move {
            let status = this.get_link_status(interface_name).await?;
            let table = lua.create_table()?;
            for (key, value) in status {
                table.set(key, value)?;
            }
            Ok(table)
        });

        methods.add_async_method("list_interfaces", |lua, this, _: ()| async move {
            let interfaces = this.list_interfaces().await?;
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
