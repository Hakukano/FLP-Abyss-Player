use axum::{
    extract::{Path, Query, State},
    response::{IntoResponse, Response},
    Json,
};
use http::StatusCode;
use serde::{Deserialize, Serialize};

use crate::{
    models::entry::Entry,
    services::Services,
    utils::meta::{Meta, MetaCmpBy},
};

#[derive(Deserialize, Serialize)]
pub struct IndexArgs {
    group_id: Option<String>,
}
pub async fn index(services: State<Services>, Query(query): Query<IndexArgs>) -> Response {
    if let Some(group_id) = query.group_id {
        Json(services.entry.read().find_by_group_id(group_id.as_str())).into_response()
    } else {
        Json(services.entry.read().all()).into_response()
    }
}

#[derive(Deserialize, Serialize)]
pub struct CreateArgs {
    group_id: String,
    path: String,
}
pub async fn create(services: State<Services>, Json(body): Json<CreateArgs>) -> Response {
    let path = std::path::Path::new(body.path.as_str());
    if !path.exists() {
        return StatusCode::NOT_FOUND.into_response();
    }
    Meta::from_path(path)
        .map(|meta| {
            services
                .entry
                .write()
                .save(Entry::new(meta, body.group_id))
                .map(|entry| (StatusCode::CREATED, Json(entry)).into_response())
                .unwrap_or_else(|err| {
                    error!("Cannot save entry: {}", err);
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
pub async fn sort(services: State<Services>, Json(body): Json<SortArgs>) -> Response {
    services.entry.write().sort(body.by, body.ascend);
    StatusCode::NO_CONTENT.into_response()
}

pub async fn show(services: State<Services>, Path(id): Path<String>) -> Response {
    services
        .entry
        .read()
        .find_by_id(id.as_str())
        .map(|entry| Json(entry).into_response())
        .unwrap_or_else(|| StatusCode::NOT_FOUND.into_response())
}

pub async fn destroy(services: State<Services>, Path(id): Path<String>) -> Response {
    services
        .entry
        .write()
        .destroy(id.as_str())
        .map(|_| StatusCode::NO_CONTENT.into_response())
        .unwrap_or_else(|_| StatusCode::NOT_FOUND.into_response())
}

#[derive(Deserialize, Serialize)]
pub struct ShiftArgs {
    offset: i64,
}
pub async fn shift(
    services: State<Services>,
    Path(id): Path<String>,
    Json(body): Json<ShiftArgs>,
) -> Response {
    let mut entries = services.entry.read().all();
    entries
        .iter()
        .position(|entry| entry.id == id)
        .map(|index| {
            let new_index = (index as i64 + body.offset)
                .max(0)
                .min(entries.len() as i64 - 1) as usize;
            let deleted = entries.remove(index);
            entries.insert(new_index, deleted);
            services.entry.write().set_all(entries);
            StatusCode::NO_CONTENT.into_response()
        })
        .unwrap_or_else(|| StatusCode::NOT_FOUND.into_response())
}
