#[macro_use]
extern crate tracing;

use tokio::net::TcpListener;
use tracing::Level;
use utils::find_available_port;

mod controllers;
mod models;
mod services;
mod utils;

#[tokio::main]
async fn main() {
    let _guard = utils::init_tracing(
        true,
        if cfg!(debug_assertions) {
            Level::DEBUG
        } else {
            Level::INFO
        },
    );

    let services = services::Services::new();

    let addr = format!(
        "127.0.0.1:{}",
        find_available_port().expect("No available port found")
    );
    let app = controllers::router(services);
    let listener = TcpListener::bind(addr.as_str()).await.unwrap();
    info!("Listening at {}", addr);
    axum::serve(listener, app).await.unwrap();
}
