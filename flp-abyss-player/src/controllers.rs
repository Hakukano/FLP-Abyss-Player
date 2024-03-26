use axum::Router;
use tower_http::services::{ServeDir, ServeFile};

use crate::services::Services;

pub mod api;

pub fn router(services: Services) -> Router {
    Router::new()
        .nest_service(
            "/",
            ServeDir::new("./assets/static")
                .not_found_service(ServeFile::new("./assets/static/index.html")),
        )
        .nest("/api", api::router())
        .with_state(services)
}
