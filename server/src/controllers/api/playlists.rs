use axum::{
    extract::Path,
    response::{IntoResponse, Response},
    Json,
};
use http::StatusCode;
use serde::{Deserialize, Serialize};

use crate::models::playlist::Playlist;

pub async fn index() -> Response {
    Json(Playlist::all()).into_response()
}

#[derive(Deserialize, Serialize)]
pub struct CreateArgs {
    name: String,
}
pub async fn create(Json(body): Json<CreateArgs>) -> Response {
    Playlist::new(body.name)
        .save()
        .map(|_| StatusCode::CREATED.into_response())
        .unwrap_or_else(|_| {
            error!("Cannot save playlist");
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        })
}

pub async fn show(Path(id): Path<String>) -> Response {
    Playlist::find(&id)
        .map(|playlist| Json(playlist).into_response())
        .unwrap_or_else(|| StatusCode::NOT_FOUND.into_response())
}

pub async fn destroy(Path(id): Path<String>) -> Response {
    Playlist::find(&id)
        .map(|playlist| playlist.destroy())
        .map(|_| StatusCode::NO_CONTENT.into_response())
        .unwrap_or_else(|| StatusCode::NOT_FOUND.into_response())
}
