use axum::{
    routing::{get, post},
    Router,
};

use crate::services::Services;

mod app_config;
mod entries;
mod groups;
mod playlists;
mod scanner;
mod session;

pub fn router() -> Router<Services> {
    Router::new()
        .route(
            "/app_config",
            get(app_config::index).put(app_config::update),
        )
        .route("/session/write", post(session::save))
        .route("/session/read", post(session::load))
        .route("/scanner", get(scanner::index))
        .route("/playlists", get(playlists::index).post(playlists::create))
        .route(
            "/playlists/:id",
            get(playlists::show).delete(playlists::destroy),
        )
        .route(
            "/groups",
            get(groups::index).post(groups::create).put(groups::sort),
        )
        .route(
            "/groups/:id",
            get(groups::show).delete(groups::destroy).put(groups::shift),
        )
        .route(
            "/entries",
            get(entries::index).post(entries::create).put(entries::sort),
        )
        .route(
            "/entries/:id",
            get(entries::show)
                .delete(entries::destroy)
                .put(entries::shift),
        )
}
