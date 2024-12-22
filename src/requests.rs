use std::collections::HashMap;

use axum::{body::Body, http::Request};

#[derive(Debug)]
pub struct RequestLua {
    pub inner_request: Request<Body>,
    pub body: String,
}
impl RequestLua {
    pub async fn new(request: Request<Body>) -> Self {
        let (parts, body) = request.into_parts();
        match axum::body::to_bytes(body, usize::MAX).await {
            Ok(bytes) => {
                let inner_request = Request::from_parts(parts, Body::from(bytes.clone()));
                let body = String::from_utf8_lossy(&bytes).to_string();

                Self {
                    inner_request,
                    body,
                }
            }

            Err(e) => {
                eprintln!("Error extracting body from request: {e:#?}");

                Self {
                    inner_request: Request::from_parts(parts, Body::empty()),
                    body: "".to_string(),
                }
            }
        }
    }
}
impl std::ops::Deref for RequestLua {
    type Target = Request<Body>;

    fn deref(&self) -> &Self::Target {
        &self.inner_request
    }
}
impl std::ops::DerefMut for RequestLua {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner_request
    }
}
unsafe impl Send for RequestLua {}
unsafe impl Sync for RequestLua {}

impl mlua::UserData for RequestLua {
    fn add_methods<M: mlua::UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("method", |_, this, ()| Ok(this.method().to_string()));
        methods.add_method("uri", |_, this, ()| Ok(this.uri().to_string()));
        methods.add_method("headers", |_, this, ()| {
            Ok(this
                .headers()
                .iter()
                .map(|(key, value)| (key.to_string(), value.to_str().unwrap_or("").to_string()))
                .collect::<HashMap<String, String>>())
        });
        methods.add_async_method("body", |_, this, ()| async move { Ok(this.body.clone()) });
    }
}
