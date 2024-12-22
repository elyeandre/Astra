#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]

use std::sync::LazyLock;

mod requests;
mod routes;

pub static LUA: LazyLock<mlua::Lua> = LazyLock::new(mlua::Lua::new);
pub static LUA_FILE_PATH: LazyLock<String> = LazyLock::new(|| {
    let lua_file = std::env::args().collect::<Vec<_>>();
    #[allow(clippy::expect_used)]
    lua_file.get(1).expect("Couldn't open the lua file").clone()
});

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
    axum::serve(listener, routes::load_routes()).await.unwrap();
}
