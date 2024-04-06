use axum::{
    extract::{Path, State},
    response::{IntoResponse, Response},
    Json,
};
use http::StatusCode;
use serde::{Deserialize, Serialize};

use crate::{models::playlist::Playlist, services::Services};

pub async fn index(services: State<Services>) -> Response {
    Json(services.playlist.read().all()).into_response()
}

#[derive(Deserialize, Serialize)]
pub struct CreateArgs {
    name: String,
}
pub async fn create(services: State<Services>, Json(body): Json<CreateArgs>) -> Response {
    let playlist = Playlist::new(body.name);
    services
        .playlist
        .write()
        .save(playlist)
        .map(|playlist| (StatusCode::CREATED, Json(playlist)).into_response())
        .unwrap_or_else(|err| {
            error!("Cannot save playlist: {}", err);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        })
}

pub async fn show(services: State<Services>, Path(id): Path<String>) -> Response {
    services
        .playlist
        .read()
        .find_by_id(id.as_str())
        .map(|playlist| Json(playlist).into_response())
        .unwrap_or_else(|| StatusCode::NOT_FOUND.into_response())
}

pub async fn destroy(services: State<Services>, Path(id): Path<String>) -> Response {
    services
        .playlist
        .write()
        .destroy(
            id.as_str(),
            services.group.write().as_mut(),
            services.entry.write().as_mut(),
        )
        .map(|_| StatusCode::NO_CONTENT.into_response())
        .unwrap_or_else(|_| StatusCode::NOT_FOUND.into_response())
}
