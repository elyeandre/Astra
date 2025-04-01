use axum::http::{HeaderMap, HeaderName, HeaderValue, StatusCode};

#[derive(Debug, Clone)]
pub enum CookieOperation {
    Add { key: String, value: String },
    Remove { key: String },
}

#[derive(Debug, Clone)]
pub struct ResponseLua {
    pub status_code: StatusCode,
    pub headers: HeaderMap,
    pub cookie_operations: Vec<CookieOperation>,
}
impl ResponseLua {
    pub fn new() -> Self {
        Self {
            status_code: StatusCode::OK,
            headers: HeaderMap::new(),
            cookie_operations: Vec::new(),
        }
    }
}
impl mlua::UserData for ResponseLua {
    fn add_methods<M: mlua::UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method_mut("set_status_code", |_, this, status_code: u16| {
            match StatusCode::from_u16(status_code) {
                Ok(status_code) => {
                    this.status_code = status_code;
                    Ok(())
                }
                Err(e) => Err(mlua::Error::runtime(format!(
                    "Could not set the response HTTP status code: {e:#?}"
                ))),
            }
        });

        methods.add_method_mut(
            "set_header",
            |_, this, (header_key, header_value): (String, String)| match HeaderName::from_lowercase(
                header_key.to_lowercase().as_bytes(),
            ) {
                Ok(header_key) => match HeaderValue::from_str(&header_value) {
                    Ok(header_value) => {
                        this.headers.insert(header_key, header_value);

                        Ok(())
                    }
                    Err(e) => Err(mlua::Error::runtime(format!(
                        "Could not set the header (value): {e:#?}"
                    ))),
                },
                Err(e) => Err(mlua::Error::runtime(format!(
                    "Could not set the header (key): {e:#?}"
                ))),
            },
        );

        methods.add_method_mut("remove_header", |_, this, header_key: String| {
            match HeaderName::from_lowercase(header_key.to_lowercase().as_bytes()) {
                Ok(header_key) => {
                    this.headers.remove(header_key);

                    Ok(())
                }
                Err(e) => Err(mlua::Error::runtime(format!(
                    "Could not set the header (key): {e:#?}"
                ))),
            }
        });

        methods.add_method("get_headers", |lua, this, ()| {
            let header_map = lua.create_table()?;

            for (key, value) in this.headers.iter() {
                header_map.set(key.as_str(), String::from_utf8_lossy(value.as_bytes()))?;
            }

            Ok(header_map)
        });

        methods.add_method_mut("set_cookie", |_, this, (key, value): (String, String)| {
            this.cookie_operations
                .push(CookieOperation::Add { key, value });

            Ok(())
        });

        methods.add_method_mut("remove_cookie", |_, this, key: String| {
            this.cookie_operations.push(CookieOperation::Remove { key });

            Ok(())
        });
    }
}
