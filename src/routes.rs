use crate::common::{LUA, LUA_FILE_PATH};
use axum::{
    body::Body,
    http::Request,
    response::IntoResponse,
    routing::{delete, get, post, put},
    Router,
};
use mlua::LuaSerdeExt;
use std::sync::LazyLock;

#[derive(Debug, Clone, Copy, mlua::FromLua, serde::Serialize, serde::Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum Method {
    Get,
    Post,
    Put,
    Delete,
}
#[derive(Debug, Clone, mlua::FromLua, PartialEq)]
pub struct Route {
    pub path: String,
    pub method: Method,
    pub function: mlua::Function,
}

pub static ROUTES: LazyLock<Vec<Route>> = LazyLock::new(|| {
    // Filter out lines that start with "require" and contain "astra.lua" or "astra.bundle.lua"
    #[allow(clippy::expect_used)]
    let user_file = std::fs::read_to_string(LUA_FILE_PATH.as_str()).expect("Couldn't read file");

    let lines: Vec<&str> = user_file.lines().collect();

    // Filter out lines that start with "require" and contain "astra.lua" or "astra.bundle.lua"
    let filtered_lines: Vec<String> = lines
        .into_iter()
        .filter(|line| {
            !(line.starts_with("require")
                && (line.contains("astra") || line.contains("astra_bundle")))
        })
        .map(|line| line.to_string()) // Convert to String
        .collect();

    // Join the filtered lines back into a single string
    let updated_content = filtered_lines.join("\n");

    #[allow(clippy::expect_used)]
    LUA.load(updated_content)
        .exec()
        .expect("Couldn't load lua file");

    if let Ok(settings) = LUA.globals().get::<mlua::Table>("Astra") {
        match settings.set("version", crate::common::get_package_version()) {
            Ok(_) => {
                if let Err(e) = LUA.globals().set("Astra", settings) {
                    println!("Error adding setting back to Astra: {e:#?}");
                }
            }
            Err(e) => {
                eprintln!("Error setting version: {e:#?}");
            }
        }
    }

    let mut routes = Vec::new();
    #[allow(clippy::unwrap_used)]
    LUA.globals()
        .get::<mlua::Table>("Astra")
        .unwrap()
        .for_each(|_key: mlua::Value, entry: mlua::Value| {
            if let Some(entry) = entry.as_table() {
                routes.push(crate::routes::Route {
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

pub async fn route(details: Route, request: Request<Body>) -> axum::response::Response {
    let request = LUA.create_userdata(crate::requests::RequestLua::new(request).await);

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

pub fn load_routes() -> Router {
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
