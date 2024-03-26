use axum::Router;

use crate::services::Services;

pub mod api;

pub fn router(services: Services) -> Router {
    Router::new()
        .nest("/api", api::router())
        .with_state(services)
}
