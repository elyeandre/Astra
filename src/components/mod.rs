use mlua::LuaSerdeExt;
mod crypto;
mod database;
mod datetime;
pub mod global;
pub mod http;
mod io;
mod regex;
mod templates;
#[cfg(unix)]
pub mod unix_socket;
#[cfg(unix)]
pub mod network;  

pub async fn register_components(lua: &mlua::Lua) -> mlua::Result<Vec<(String, String)>> {
    let global = global::register_to_lua(lua);
    http::server::register_to_lua(lua)?;
    http::client::HTTPClientRequest::register_to_lua(lua)?;
    let database = database::Database::register_to_lua(lua)?;
    let datetime = datetime::LuaDateTime::register_to_lua(lua)?;
    let crypto = crypto::register_to_lua(lua)?;
    let fileio = io::register_to_lua(lua)?;
    let templates = templates::TemplatingEngine::register_to_lua(lua)?;
    let regex = regex::LuaRegex::register_to_lua(lua)?;

    let mut components: Vec<(String, String)> = vec![
        ("global.lua".to_string(), global.to_string()),
        ("http.lua".to_string(), http::type_definitions()),
        ("database.lua".to_string(), database.to_string()),
        ("crypto.lua".to_string(), crypto.to_string()),
        ("io.lua".to_string(), fileio.to_string()),
        ("templates.lua".to_string(), templates.to_string()),
        ("regex.lua".to_string(), regex.to_string()),
        ("datetime.lua".to_string(), datetime.to_string()),
    ];

    #[cfg(unix)]
    {
        components.push((
            "unix_socket.lua".to_string(),
            unix_socket::UnixSocketComponent::lua_code().to_string()
        ));
        
        // Add network component
        components.push((
            "network.lua".to_string(),
            network::NetworkComponent::lua_code().to_string()
        ));
    }

    Ok(components)
}

#[derive(Debug, Clone)]
pub struct BodyLua {
    #[allow(unused)]
    pub body: bytes::Bytes,
    pub body_string: String,
}
impl BodyLua {
    pub fn new(bytes: bytes::Bytes) -> Self {
        let body_string = String::from_utf8_lossy(&bytes).to_string();

        Self {
            body: bytes,
            body_string,
        }
    }
}
impl mlua::UserData for BodyLua {
    fn add_methods<M: mlua::UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("text", |_, this, ()| Ok(this.body_string.clone()));

        methods.add_method("json", |lua, this, ()| {
            match serde_json::from_str::<serde_json::Value>(&this.body_string) {
                Ok(body_json) => Ok(lua.to_value(&body_json)?),
                Err(e) => Err(mlua::Error::runtime(format!(
                    "Could not parse the body as JSON: {e:#?}"
                ))),
            }
        });
    }
}

#[allow(unused)]
pub mod macros {
    macro_rules! impl_deref {
        ($struct:ty,$type:ty) => {
            impl std::ops::Deref for $struct {
                type Target = $type;

                fn deref(&self) -> &Self::Target {
                    &self.0
                }
            }
            impl std::ops::DerefMut for $struct {
                fn deref_mut(&mut self) -> &mut Self::Target {
                    &mut self.0
                }
            }
        };
    }

    macro_rules! impl_deref_field {
        ($struct:ty,$type:ty,$field:ident) => {
            impl std::ops::Deref for $struct {
                type Target = $type;

                fn deref(&self) -> &Self::Target {
                    &self.$field
                }
            }
            impl std::ops::DerefMut for $struct {
                fn deref_mut(&mut self) -> &mut Self::Target {
                    &mut self.$field
                }
            }
        };
    }

    pub(crate) use impl_deref;
    pub(crate) use impl_deref_field;
}
