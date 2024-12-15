use axum::{
    extract::Query,
    routing::{delete, get, post, put},
    Router,
};
use mlua::LuaSerdeExt;
use serde_json::Value;
use std::sync::LazyLock;

#[derive(Debug, Clone, Copy, mlua::FromLua, serde::Serialize, serde::Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
enum Method {
    Get,
    Post,
    Put,
    Delete,
}
#[derive(Debug, Clone, Copy, mlua::FromLua, serde::Serialize, serde::Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
enum Returns {
    Plain,
    Json,
    StatusCode,
}
#[derive(Debug, Clone, mlua::FromLua, PartialEq)]
struct Route {
    path: String,
    method: Method,
    returns: Returns,
    function: mlua::Function,
}

static LUA_FILE_PATH: LazyLock<String> = LazyLock::new(|| {
    let lua_file = std::env::args().collect::<Vec<_>>();
    lua_file.get(1).expect("Couldn't open the lua file").clone()
});
static LUA: LazyLock<mlua::Lua> = LazyLock::new(mlua::Lua::new);
static ROUTES: LazyLock<Vec<Route>> = LazyLock::new(|| {
    LUA.load(std::fs::read_to_string(LUA_FILE_PATH.as_str()).expect("Couldn't read file"))
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
                returns: LUA.from_value(entry.get("returns")?)?,
                function: entry.get::<mlua::Function>("func")?,
            });

            Ok(())
        })
        .unwrap();

    println!("Routes: {:#?}", routes);

    routes
});

async fn route(details: Route) -> impl axum::response::IntoResponse {
    let result = details.function.call::<mlua::Value>(());
    match result {
        Ok(val) => Ok("".to_string()),
        Err(e) => {
            eprintln!("Route Calling Error: {e}");

            Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

fn load_routes() -> Router {
    let mut router = Router::new();
    for route_values in ROUTES.clone() {
        router = router.route(
            route_values.path.clone().as_str(),
            match route_values.method {
                Method::Get => get(|Query(query): Query<Value>| route(route_values)),
                Method::Post => post(|Query(query): Query<Value>| route(route_values)),
                Method::Put => put(|Query(query): Query<Value>| route(route_values)),
                Method::Delete => delete(|Query(query): Query<Value>| route(route_values)),
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
