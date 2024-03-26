#[macro_use]
extern crate tracing;

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

    let services = services::Services::new();

    let app = controllers::router(services);
    let listener = TcpListener::bind("0.0.0.0:44444").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
