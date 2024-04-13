use axum::{
    extract::{Path, Query},
    response::{IntoResponse, Response},
    Json,
};
use http::StatusCode;
use serde::{Deserialize, Serialize};

use crate::{
    models::entry::Entry,
    utils::meta::{Meta, MetaCmpBy},
};

#[derive(Deserialize, Serialize)]
pub struct IndexArgs {
    group_id: Option<String>,
}
pub async fn index(Query(query): Query<IndexArgs>) -> Response {
    if let Some(group_id) = query.group_id {
        Json(Entry::find_by_group_id(&group_id)).into_response()
    } else {
        Json(Entry::all()).into_response()
    }
}

#[derive(Deserialize, Serialize)]
pub struct CreateArgs {
    group_id: String,
    path: String,
}
pub async fn create(Json(body): Json<CreateArgs>) -> Response {
    let path = std::path::Path::new(body.path.as_str());
    if !path.exists() {
        return StatusCode::NOT_FOUND.into_response();
    }
    Meta::from_path(path)
        .map(|meta| {
            Entry::new(meta, body.group_id)
                .save()
                .map(|entry| (StatusCode::CREATED, Json(entry)).into_response())
                .unwrap_or_else(|_| {
                    error!("Cannot save entry");
                    StatusCode::INTERNAL_SERVER_ERROR.into_response()
                })
        })
        .unwrap_or_else(|err| {
            error!("Cannot read entry meta: {}", err);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        })
}

#[derive(Deserialize, Serialize)]
pub struct SortArgs {
    by: MetaCmpBy,
    ascend: bool,
}
pub async fn sort(Json(body): Json<SortArgs>) -> Response {
    crate::services::entry::sort(body.by, body.ascend);
    StatusCode::NO_CONTENT.into_response()
}

pub async fn show(Path(id): Path<String>) -> Response {
    Entry::find(&id)
        .map(|entry| Json(entry).into_response())
        .unwrap_or_else(|| StatusCode::NOT_FOUND.into_response())
}

pub async fn destroy(Path(id): Path<String>) -> Response {
    Entry::find(&id)
        .map(|entry| entry.destroy())
        .map(|_| StatusCode::NO_CONTENT.into_response())
        .unwrap_or_else(|| StatusCode::NOT_FOUND.into_response())
}

#[derive(Deserialize, Serialize)]
pub struct ShiftArgs {
    offset: i64,
}
pub async fn shift(Path(id): Path<String>, Json(body): Json<ShiftArgs>) -> Response {
    crate::services::entry::shift(id.as_str(), body.offset)
        .map(|_| StatusCode::NO_CONTENT.into_response())
        .unwrap_or_else(|_| StatusCode::NOT_FOUND.into_response())
}
