use axum::Router;
use tower_http::{
    services::{ServeDir, ServeFile},
    trace::TraceLayer,
};

use crate::{services::Services, utils::fs::public_path};

mod api;
mod stream;

pub fn router(services: Services) -> Router {
    Router::new()
        .nest_service(
            "/",
            ServeDir::new(public_path())
                .not_found_service(ServeFile::new(public_path().join("index.html"))),
        )
        .nest("/api", api::router())
        .nest("/stream", stream::router())
        .layer(TraceLayer::new_for_http())
        .with_state(services)
}
