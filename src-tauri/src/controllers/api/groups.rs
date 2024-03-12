use std::path::Path;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{
    models::group::Group,
    services::group::GroupService,
    utils::meta::{Meta, MetaCmpBy},
};

use super::{ApiResult, FromArgs, Response};

#[derive(Deserialize, Serialize)]
struct IndexArgs {
    playlist_id: Option<String>,
}
impl FromArgs for IndexArgs {}
pub fn index(args: Value, group_service: &dyn GroupService) -> ApiResult {
    let args = IndexArgs::from_args(args)?;
    if let Some(playlist_id) = args.playlist_id {
        Response::ok(group_service.find_by_playlist_id(playlist_id.as_str()))
    } else {
        Response::ok(group_service.all())
    }
}

#[derive(Deserialize, Serialize)]
struct CreateArgs {
    playlist_id: String,
    path: String,
}
impl FromArgs for CreateArgs {}
pub fn create(args: Value, group_service: &mut dyn GroupService) -> ApiResult {
    let args = CreateArgs::from_args(args)?;

    if group_service
        .find_by_playlist_id(args.playlist_id.as_str())
        .into_iter()
        .any(|group| group.meta.path == args.path.as_str())
    {
        return Err(Response::conflict());
    }

    let path = Path::new(args.path.as_str());
    if !path.exists() {
        return Err(Response::not_found());
    }
    let meta = Meta::from_path(path).map_err(|err| {
        error!("Cannot read group meta: {}", err);
        Response::internal_server_error()
    })?;
    let group = Group::new(meta, args.playlist_id);
    group_service
        .save(group)
        .map_err(|err| {
            error!("Cannot save group: {}", err);
            Response::internal_server_error()
        })
        .and_then(Response::created)
}

#[derive(Deserialize, Serialize)]
struct SortArgs {
    by: MetaCmpBy,
    ascend: bool,
}
impl FromArgs for SortArgs {}
pub fn sort(args: Value, group_service: &mut dyn GroupService) -> ApiResult {
    let args = SortArgs::from_args(args)?;
    group_service.sort(args.by, args.ascend);
    Ok(Response::no_content())
}

pub fn show(id: &str, group_service: &dyn GroupService) -> ApiResult {
    Response::ok(
        group_service
            .find_by_id(id)
            .ok_or_else(Response::not_found)?,
    )
}

pub fn destroy(id: &str, group_service: &mut dyn GroupService) -> ApiResult {
    group_service
        .destroy(id)
        .map_err(|_| Response::not_found())?;
    Ok(Response::no_content())
}
