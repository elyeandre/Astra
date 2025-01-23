use mlua::{LuaSerdeExt, UserData};
use reqwest::{Client, RequestBuilder};
use std::collections::HashMap;

// TODO: Add HTTPClientResponse and change the below as HTTPClientRequest.
// TODO: HTTPClientRequest must be chained setter
// TODO: HTTPClientResponse will have the current UserData trait implementations

#[allow(clippy::upper_case_acronyms)]
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, serde::Deserialize, mlua::FromLua)]
pub enum HTTPRequestMethod {
    GET,
    POST,
    PUT,
    PATCH,
    DELETE,
    HEAD,
}

#[derive(Debug, Clone)]
pub struct HTTPClientRequest {
    pub url: String,
    pub method: String,
    pub headers: HashMap<String, String>,
    pub body: Option<String>,
    pub body_json: Option<serde_json::Value>,
    pub form: HashMap<String, String>,
    pub callback: Option<mlua::Function>,
}
impl crate::utils::LuaUtils for HTTPClientRequest {
    async fn register_to_lua(lua: &mlua::Lua) -> mlua::Result<()> {
        let function = lua.create_function(|_, url: String| {
            Ok(Self {
                url,
                method: "GET".to_string(),
                headers: HashMap::new(),
                body: None,
                body_json: None,
                form: HashMap::new(),
                callback: None,
            })
        })?;
        lua.globals().set("http_request", function)
    }
}
impl HTTPClientRequest {
    // async fn execute_request(client: reqwest::RequestBuilder, callback: Option<mlua::Function>) {
    //     let response = match client.send().await {
    //         Ok(response) => {
    //             let status_code = response.status().as_u16();
    //             let remote_address = response.remote_addr().map(|i| i.to_string());
    //             let headers = response
    //                 .headers()
    //                 .iter()
    //                 .map(|(key, value)| {
    //                     (
    //                         key.to_string(),
    //                         String::from_utf8_lossy(value.as_bytes()).to_string(),
    //                     )
    //                 })
    //                 .collect::<std::collections::HashMap<String, String>>();

    //             match response.bytes().await {
    //                 Ok(bytes) => Ok(HTTPClientRequest {
    //                     body: BodyLua::new(bytes),
    //                     status_code,
    //                     headers,
    //                     remote_address,
    //                 }),

    //                 Err(e) => Err(mlua::Error::runtime(format!(
    //                     "Error executing the HTTP Request body: {e}"
    //                 ))),
    //             }
    //         }
    //         Err(e) => Err(mlua::Error::runtime(format!(
    //             "Error executing the HTTP Request: {e}"
    //         ))),
    //     };

    //     if let Some(callback) = callback {
    //         if let Err(e) = callback.call::<()>(response) {
    //             eprintln!("Error running the HTTP Request callback: {e}");
    //         }
    //     }
    // }

    pub fn request_builder(&self) -> RequestBuilder {
        let mut client = match self.method.to_uppercase().as_str() {
            "POST" => Client::new().post(&self.url),
            "PATCH" => Client::new().patch(&self.url),
            "PUT" => Client::new().put(&self.url),
            "DELETE" => Client::new().delete(&self.url),
            "HEAD" => Client::new().head(&self.url),
            _ => Client::new().get(&self.url),
        };

        client = if let Some(body) = &self.body {
            client.body(body.clone())
        } else if let Some(body) = &self.body_json {
            client.json(&body)
        } else {
            client
        };

        if !self.headers.is_empty() {
            for (key, value) in self.headers.iter() {
                client = client.header(key, value);
            }
        }

        if !self.form.is_empty() {
            client = client.form(&self.form);
        }

        client
    }
}
impl UserData for HTTPClientRequest {
    fn add_methods<M: mlua::UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("set_method", |_, this, method: String| {
            let mut request = this.clone();
            request.method = method;

            Ok(request)
        });

        methods.add_method_mut("set_header", |_, this, (key, value): (String, String)| {
            let mut request = this.clone();
            request.headers.insert(key, value);

            Ok(request)
        });

        methods.add_method_mut(
            "set_headers",
            |_, this, headers: HashMap<String, String>| {
                let mut request = this.clone();
                request.headers = headers;

                Ok(request)
            },
        );

        methods.add_method_mut("set_form", |_, this, (key, value): (String, String)| {
            let mut request = this.clone();
            request.form.insert(key, value);

            Ok(request)
        });

        methods.add_method_mut("set_forms", |_, this, form: HashMap<String, String>| {
            let mut request = this.clone();
            request.form = form;

            Ok(request)
        });

        methods.add_method_mut("set_body", |_, this, body: String| {
            let mut request = this.clone();
            request.body = Some(body);

            Ok(request)
        });

        methods.add_method_mut("set_json", |lua, this, body: mlua::Value| {
            let mut request = this.clone();
            request.body_json = lua.from_value::<serde_json::Value>(body).ok();

            Ok(request)
        });

        methods.add_async_method("execute", |_, this, ()| async move {
            let request = this.request_builder();
            match request.send().await {
                Ok(response) => {
                    println!("{response:#?}");
                    //
                    Ok(())
                }
                Err(e) => Err(mlua::Error::runtime(format!(
                    "HTTP Request did not execute successfully: {e}"
                ))),
            }
        });
    }
}
