use axum::{
    extract::{Path, Query, State},
    response::{IntoResponse, Response},
    Json,
};
use http::StatusCode;
use serde::{Deserialize, Serialize};

use crate::{
    models::group::Group,
    services::Services,
    utils::meta::{Meta, MetaCmpBy},
};

#[derive(Deserialize, Serialize)]
pub struct IndexArgs {
    playlist_id: Option<String>,
}
pub async fn index(services: State<Services>, Query(query): Query<IndexArgs>) -> Response {
    if let Some(playlist_id) = query.playlist_id {
        Json(
            services
                .group
                .read()
                .find_by_playlist_id(playlist_id.as_str()),
        )
        .into_response()
    } else {
        Json(services.group.read().all()).into_response()
    }
}

#[derive(Deserialize, Serialize)]
pub struct CreateArgs {
    playlist_id: String,
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
                .group
                .write()
                .save(Group::new(meta, body.playlist_id))
                .map(|group| (StatusCode::CREATED, Json(group)).into_response())
                .unwrap_or_else(|err| {
                    error!("Cannot save group: {}", err);
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
pub async fn sort(services: State<Services>, Json(body): Json<SortArgs>) -> Response {
    services.group.write().sort(body.by, body.ascend);
    StatusCode::NO_CONTENT.into_response()
}

pub async fn show(services: State<Services>, Path(id): Path<String>) -> Response {
    services
        .group
        .read()
        .find_by_id(id.as_str())
        .map(|group| Json(group).into_response())
        .unwrap_or_else(|| StatusCode::NOT_FOUND.into_response())
}

pub async fn destroy(services: State<Services>, Path(id): Path<String>) -> Response {
    services
        .group
        .write()
        .destroy(id.as_str(), services.entry.write().as_mut())
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
    let mut groups = services.group.read().all();
    groups
        .iter()
        .position(|group| group.id == id)
        .map(|index| {
            let new_index = (index as i64 + body.offset)
                .max(0)
                .min(groups.len() as i64 - 1) as usize;
            let deleted = groups.remove(index);
            groups.insert(new_index, deleted);
            services.group.write().set_all(groups);
            StatusCode::NO_CONTENT.into_response()
        })
        .unwrap_or_else(|| StatusCode::NOT_FOUND.into_response())
}
