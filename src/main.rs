#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]

mod common;
mod requests;
mod routes;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let mut listener_address = "127.0.0.1:8080".to_string();

    if let Ok(settings) = common::LUA.globals().get::<mlua::Table>("Astra") {
        if let Ok(hostname) = settings.get::<String>("hostname") {
            if let Ok(port) = settings.get::<u16>("port") {
                listener_address = format!("{hostname}:{port}");
            }
        }
    }

    #[allow(clippy::unwrap_used)]
    let listener = tokio::net::TcpListener::bind(listener_address.clone())
        .await
        .unwrap();

    println!("🚀 Listening at: http://{listener_address}");

    #[allow(clippy::unwrap_used)]
    axum::serve(listener, routes::load_routes()).await.unwrap();
}
