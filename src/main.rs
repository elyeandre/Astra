#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]

use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod common;
mod requests;
mod responses;
mod routes;
mod utils;

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                format!("{}=debug,tower_http=debug", env!("CARGO_CRATE_NAME")).into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    common::init().await;

    // get the metrics for current tokio tasks
    let metrics = tokio::runtime::Handle::current().metrics();
    loop {
        // wait for them to finish
        let alive_tasks = metrics.num_alive_tasks();
        if alive_tasks == 0 {
            break;
        }
    }
}
