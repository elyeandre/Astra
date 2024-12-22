#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]

use axum::{
    body::Body,
    http::Request,
    response::IntoResponse,
    routing::{delete, get, post, put},
    Router,
};
use mlua::LuaSerdeExt;
use std::{collections::HashMap, sync::LazyLock};

#[derive(Debug, Clone, Copy, mlua::FromLua, serde::Serialize, serde::Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
enum Method {
    Get,
    Post,
    Put,
    Delete,
}
#[derive(Debug, Clone, mlua::FromLua, PartialEq)]
struct Route {
    path: String,
    method: Method,
    function: mlua::Function,
}

static LUA_FILE_PATH: LazyLock<String> = LazyLock::new(|| {
    let lua_file = std::env::args().collect::<Vec<_>>();
    #[allow(clippy::expect_used)]
    lua_file.get(1).expect("Couldn't open the lua file").clone()
});
static LUA: LazyLock<mlua::Lua> = LazyLock::new(mlua::Lua::new);
static ROUTES: LazyLock<Vec<Route>> = LazyLock::new(|| {
    let lua_prelude = include_str!("../lua/astra_bundle.lua");
    #[allow(clippy::expect_used)]
    LUA.load(lua_prelude).exec().expect("Couldn't add prelude");

    // Filter out lines that start with "require" and contain "astra.lua" or "astra.bundle.lua"
    #[allow(clippy::expect_used)]
    let user_file = std::fs::read_to_string(LUA_FILE_PATH.as_str()).expect("Couldn't read file");

    let lines: Vec<&str> = user_file.lines().collect();

    // Filter out lines that start with "require" and contain "astra.lua" or "astra.bundle.lua"
    let filtered_lines: Vec<String> = lines
        .into_iter()
        .filter(|line| {
            !(line.starts_with("require")
                && (line.contains("astra.lua") || line.contains("astra_bundle.lua")))
        })
        .map(|line| line.to_string()) // Convert to String
        .collect();

    // Join the filtered lines back into a single string
    let updated_content = filtered_lines.join("\n");

    #[allow(clippy::expect_used)]
    LUA.load(updated_content)
        .exec()
        .expect("Couldn't load lua file");

    let mut routes = Vec::new();
    #[allow(clippy::unwrap_used)]
    LUA.globals()
        .get::<mlua::Table>("Astra")
        .unwrap()
        .for_each(|_key: mlua::Value, entry: mlua::Value| {
            if let Some(entry) = entry.as_table() {
                routes.push(Route {
                    path: LUA.from_value(entry.get("path")?)?,
                    method: LUA.from_value(entry.get("method")?)?,
                    function: entry.get::<mlua::Function>("func")?,
                });
            }

            Ok(())
        })
        .unwrap();

    routes
});

#[derive(Debug)]
struct RequestLua {
    inner_request: Request<Body>,
    body: String,
}
impl RequestLua {
    async fn new(request: Request<Body>) -> Self {
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

async fn route(details: Route, request: Request<Body>) -> axum::response::Response {
    let request = LUA.create_userdata(RequestLua::new(request).await);

    let result = details.function.call_async::<mlua::Value>(request).await;
    match result {
        Ok(value) => match value {
            mlua::Value::String(plain) => plain.to_string_lossy().into_response(),
            mlua::Value::Table(_) => match LUA.from_value::<serde_json::Value>(value.clone()) {
                Ok(result) => axum::Json(result).into_response(),
                Err(e) => {
                    eprintln!("Result Parsing Error: {e}");

                    axum::http::StatusCode::INTERNAL_SERVER_ERROR.into_response()
                }
            },
            _ => axum::http::StatusCode::OK.into_response(),
        },
        Err(e) => {
            eprintln!("Route Calling Error: {e}");

            axum::http::StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

fn load_routes() -> Router {
    let mut router = Router::new();

    for route_values in ROUTES.clone() {
        router = router.route(
            route_values.path.clone().as_str(),
            match route_values.method {
                Method::Get => get(|request: Request<Body>| route(route_values, request)),
                Method::Post => post(|request: Request<Body>| route(route_values, request)),
                Method::Put => put(|request: Request<Body>| route(route_values, request)),
                Method::Delete => delete(|request: Request<Body>| route(route_values, request)),
            },
        );
    }

    router
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    #[allow(clippy::unwrap_used)]
    let listener = tokio::net::TcpListener::bind("127.0.0.1:20001")
        .await
        .unwrap();

    println!("🚀 Listening at: http://127.0.0.1:20001");

    #[allow(clippy::unwrap_used)]
    axum::serve(listener, load_routes()).await.unwrap();
}
