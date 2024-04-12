use axum::{
    extract::{Path, Query},
    response::{IntoResponse, Response},
    Json,
};
use http::StatusCode;
use serde::{Deserialize, Serialize};

use crate::{
    models::group::Group,
    utils::meta::{Meta, MetaCmpBy},
};

#[derive(Deserialize, Serialize)]
pub struct IndexArgs {
    playlist_id: Option<String>,
}
pub async fn index(Query(query): Query<IndexArgs>) -> Response {
    if let Some(playlist_id) = query.playlist_id {
        Json(Group::find_by_playlist_id(&playlist_id)).into_response()
    } else {
        Json(Group::all()).into_response()
    }
}

#[derive(Deserialize, Serialize)]
pub struct CreateArgs {
    playlist_id: String,
    path: String,
}
pub async fn create(Json(body): Json<CreateArgs>) -> Response {
    let path = std::path::Path::new(body.path.as_str());
    if !path.exists() {
        return StatusCode::NOT_FOUND.into_response();
    }
    Meta::from_path(path)
        .map(|meta| {
            Group::new(meta, body.playlist_id)
                .save()
                .map(|_| StatusCode::CREATED.into_response())
                .unwrap_or_else(|_| {
                    error!("Cannot save group");
                    StatusCode::INTERNAL_SERVER_ERROR.into_response()
                })
        })
        .unwrap_or_else(|err| {
            error!("Cannot read group meta: {}", err);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        })
}

#[derive(Deserialize, Serialize)]
pub struct SortArgs {
    by: MetaCmpBy,
    ascend: bool,
}
pub async fn sort(Json(body): Json<SortArgs>) -> Response {
    crate::services::group::sort(body.by, body.ascend);
    StatusCode::NO_CONTENT.into_response()
}

pub async fn show(Path(id): Path<String>) -> Response {
    Group::find(&id)
        .map(|group| Json(group).into_response())
        .unwrap_or_else(|| StatusCode::NOT_FOUND.into_response())
}

pub async fn destroy(Path(id): Path<String>) -> Response {
    Group::find(&id)
        .map(|group| group.destroy())
        .map(|_| StatusCode::NO_CONTENT.into_response())
        .unwrap_or_else(|| StatusCode::NOT_FOUND.into_response())
}

#[derive(Deserialize, Serialize)]
pub struct ShiftArgs {
    offset: i64,
}
pub async fn shift(Path(id): Path<String>, Json(body): Json<ShiftArgs>) -> Response {
    crate::services::group::shift(id.as_str(), body.offset)
        .map(|_| StatusCode::NO_CONTENT.into_response())
        .unwrap_or_else(|_| StatusCode::NOT_FOUND.into_response())
}
