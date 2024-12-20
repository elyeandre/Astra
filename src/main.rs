use axum::{
    body::Body,
    extract::Request,
    response::IntoResponse,
    routing::{delete, get, post, put},
    Router,
};
use mlua::LuaSerdeExt;
use serde_json::Value;
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
    lua_file.get(1).expect("Couldn't open the lua file").clone()
});
static LUA: LazyLock<mlua::Lua> = LazyLock::new(mlua::Lua::new);
static ROUTES: LazyLock<Vec<Route>> = LazyLock::new(|| {
    let lua_prelude = include_str!("../lua/astra.bundle.lua");
    LUA.load(lua_prelude).exec().expect("Couldn't add prelude");

    // Filter out lines that start with "require" and contain "astra.lua" or "astra.bundle.lua"
    let user_file = std::fs::read_to_string(LUA_FILE_PATH.as_str()).expect("Couldn't read file");

    let lines: Vec<&str> = user_file.lines().collect();

    // Filter out lines that start with "require" and contain "astra.lua" or "astra.bundle.lua"
    let filtered_lines: Vec<String> = lines
        .into_iter()
        .filter(|line| {
            !(line.starts_with("require")
                && (line.contains("astra.lua") || line.contains("astra.bundle.lua")))
        })
        .map(|line| line.to_string()) // Convert to String
        .collect();

    // Join the filtered lines back into a single string
    let updated_content = filtered_lines.join("\n");

    LUA.load(updated_content)
        .exec()
        .expect("Couldn't load lua file");

    let mut routes = Vec::new();
    LUA.globals()
        .get::<mlua::Table>("Astra")
        .unwrap()
        .for_each(|_key: i32, entry: mlua::Table| {
            routes.push(Route {
                path: LUA.from_value(entry.get("path")?)?,
                method: LUA.from_value(entry.get("method")?)?,
                function: entry.get::<mlua::Function>("func")?,
            });

            Ok(())
        })
        .unwrap();

    routes
});

async fn parse_request(req: Request<Body>) -> Result<axum::Json<Value>, axum::http::StatusCode> {
    // Extract request metadata: method, URI, and headers
    let method = req.method().to_string();
    let uri = req.uri().to_string();
    let headers: HashMap<String, String> = req
        .headers()
        .iter()
        .map(|(key, value)| (key.to_string(), value.to_str().unwrap_or("").to_string()))
        .collect();

    // Convert the request body into a byte vector
    let body_bytes = axum::body::to_bytes(req.into_body(), usize::MAX)
        .await
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    let body_string = String::from_utf8_lossy(&body_bytes).to_string();

    // Build the full JSON object with all the request data
    let request_json = serde_json::json!({
        "method": method,
        "uri": uri,
        "headers": headers,
        "body": body_string,
    });

    // Return the constructed JSON as the response
    Ok(axum::Json(request_json))
}

async fn route(details: Route, request: Request<Body>) -> axum::response::Response {
    let request = if let Ok(request) = parse_request(request).await {
        request.0
    } else {
        Value::Null
    };

    let parsed_request = LUA.to_value(&request);

    let result = details.function.call::<mlua::Value>(parsed_request);
    match result {
        Ok(value) => match LUA.from_value::<serde_json::Value>(value) {
            Ok(result) => match result {
                serde_json::Value::String(plain) => plain.into_response(),
                serde_json::Value::Object(_) => axum::Json(result).into_response(),
                _ => axum::http::StatusCode::OK.into_response(),
            },
            Err(e) => {
                eprintln!("Result Parsing Error: {e}");

                axum::http::StatusCode::INTERNAL_SERVER_ERROR.into_response()
            }
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

    #[allow(clippy::unwrap_used)]
    axum::serve(listener, load_routes()).await.unwrap();
}
