use std::path::Path;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{models::entry::Entry, services::entry::EntryService, utils::meta::Meta};

use super::{ApiResult, Response};

#[derive(Deserialize, Serialize)]
struct IndexArgs {
    group_id: Option<String>,
}
pub fn index(args: Value, entry_service: &dyn EntryService) -> ApiResult {
    let args: IndexArgs =
        serde_json::from_value(args).map_err(|err| Response::bad_request(err.to_string()))?;
    if let Some(group_id) = args.group_id {
        Response::ok(entry_service.find_by_group_id(group_id.as_str()))
    } else {
        Response::ok(entry_service.all())
    }
}

#[derive(Deserialize, Serialize)]
struct CreateArgs {
    group_id: String,
    path: String,
}
pub fn create(args: Value, entry_service: &mut dyn EntryService) -> ApiResult {
    let args: CreateArgs =
        serde_json::from_value(args).map_err(|err| Response::bad_request(err.to_string()))?;

    if entry_service
        .find_by_group_id(args.group_id.as_str())
        .into_iter()
        .find(|entry| entry.meta.path == args.path.as_str())
        .is_some()
    {
        return Err(Response::conflict());
    }

    let path = Path::new(args.path.as_str());
    if !path.exists() {
        return Err(Response::not_found());
    }
    let meta = Meta::from_path(path).map_err(|err| {
        error!("Cannot read entry meta: {}", err);
        Response::internal_server_error()
    })?;
    let entry = Entry::new(meta, args.group_id);
    entry_service
        .save(entry)
        .map_err(|err| {
            error!("Cannot save entry: {}", err);
            Response::internal_server_error()
        })
        .and_then(|g| Response::created(g))
}

pub fn show(id: &str, entry_service: &dyn EntryService) -> ApiResult {
    Response::ok(
        entry_service
            .find_by_id(id)
            .ok_or_else(|| Response::not_found())?,
    )
}

pub fn destroy(id: &str, entry_service: &mut dyn EntryService) -> ApiResult {
    entry_service
        .destroy(id)
        .map_err(|_| Response::not_found())?;
    Ok(Response::no_content())
}
