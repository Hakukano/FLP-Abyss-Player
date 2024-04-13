#[macro_use]
extern crate tracing;

#[macro_use]
extern crate async_trait;

use tokio::net::TcpListener;
use tracing::Level;

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

    let addr = "0.0.0.0:44444".to_string();
    let app = controllers::router();
    let listener = TcpListener::bind(addr.as_str()).await.unwrap();
    info!("Listening at {}", addr);
    axum::serve(listener, app).await.unwrap();
}
