use axum::Router;
use tower_http::{
    services::{ServeDir, ServeFile},
    trace::TraceLayer,
};

use crate::services::Services;

mod api;
mod stream;

pub fn router(services: Services) -> Router {
    Router::new()
        .nest_service(
            "/",
            ServeDir::new("./assets/static")
                .not_found_service(ServeFile::new("./assets/static/index.html")),
        )
        .nest("/api", api::router())
        .nest("/stream", stream::router())
        .layer(TraceLayer::new_for_http())
        .with_state(services)
}
