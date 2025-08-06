use mlua::{Lua, Result, UserData, UserDataMethods};
use std::path::Path;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::UnixStream,
    time,
};
use std::time::Duration;

/// Represents a Unix domain socket connection
pub struct UnixSocket {
    stream: Option<UnixStream>,
}

impl UnixSocket {
    pub async fn connect(path: String) -> Result<Self> {
        if !Self::is_safe_path(&path) {
            return Err(mlua::Error::runtime("Access to this socket path is restricted"));
        }


        match time::timeout(
            Duration::from_secs(2),
            UnixStream::connect(Path::new(&path)),
        )
        .await
        {
            Ok(Ok(stream)) => {
                Ok(Self {
                    stream: Some(stream),
                })
            },
            Ok(Err(e)) => {
                Err(mlua::Error::runtime(format!(
                    "Connection failed: {}",
                    e
                )))
            },
            Err(_) => {
                Err(mlua::Error::runtime("Connection timed out"))
            },
        }
    }

    fn is_safe_path(path: &str) -> bool {
        let allowed_prefixes = [
            "/var/run/",
            "/tmp/",
            "/run/user/"
        ];
        allowed_prefixes.iter().any(|prefix| path.starts_with(prefix))
    }

    pub async fn send(&mut self, data: Vec<u8>) -> Result<()> {
        let stream = match self.stream.as_mut() {
            Some(s) => s,
            None => return Err(mlua::Error::runtime("Socket not connected")),
        };

        match time::timeout(Duration::from_secs(5), stream.write_all(&data)).await {
            Ok(Ok(_)) => Ok(()),
            Ok(Err(e)) => Err(mlua::Error::runtime(format!("Write failed: {}", e))),
            Err(_) => Err(mlua::Error::runtime("Write operation timed out")),
        }
    }

    pub async fn receive(&mut self, buf_size: usize) -> Result<Vec<u8>> {
        let stream = match self.stream.as_mut() {
            Some(s) => s,
            None => return Err(mlua::Error::runtime("Socket not connected")),
        };

        let mut buf = vec![0u8; buf_size];
        match time::timeout(Duration::from_secs(5), stream.read(&mut buf)).await {
            Ok(Ok(bytes_read)) => {
                buf.truncate(bytes_read);
                Ok(buf)
            }
            Ok(Err(e)) => Err(mlua::Error::runtime(format!("Read failed: {}", e))),
            Err(_) => Err(mlua::Error::runtime("Read operation timed out")),
        }
    }

    pub async fn close(&mut self) -> Result<()> {
        if let Some(mut stream) = self.stream.take() {
            let _ = time::timeout(Duration::from_secs(1), stream.shutdown()).await;
        }
        Ok(())
    }

    pub fn is_connected(&self) -> bool {
        self.stream.is_some()
    }
}

impl UserData for UnixSocket {
    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        methods.add_async_method_mut("send", |_, mut this, data: Vec<u8>| async move {
            this.send(data).await
        });
        
        methods.add_async_method_mut("receive", |_, mut this, buf_size: usize| async move {
            this.receive(buf_size).await
        });
        
        methods.add_async_method_mut("close", |_, mut this, _: ()| async move {
            this.close().await
        });
        
        methods.add_method("is_connected", |_, this, _: ()| {
            Ok(this.is_connected())
        });
    }
}

pub struct UnixSocketComponent;

impl UnixSocketComponent {
    
    pub async fn register_to_lua(lua: &Lua) -> mlua::Result<()> {
        // Get or create Astra table
        let astra_table: mlua::Table = match lua.globals().get("Astra") {
            Ok(table) => table,
            Err(_) => {
                let table = lua.create_table()?;
                lua.globals().set("Astra", table.clone())?;
                table

            }

        };

        // Create unix subtable
        let unix_table = lua.create_table()?;
        unix_table.set(
            "connect",
            lua.create_async_function(|lua, path: String| async move {

                UnixSocket::connect(path).await
                    .and_then(|s| lua.create_userdata(s))

            })?,

        )?;
        astra_table.set("unix", unix_table)?;

        Ok(())

    }

    pub fn lua_code() -> &'static str {
        include_str!("unix_socket.lua")

    }
}
