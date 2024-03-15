use std::path::Path;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{
    models::entry::Entry,
    services::entry::EntryService,
    utils::meta::{Meta, MetaCmpBy},
};

use super::{ApiResult, FromArgs, Response};

#[derive(Deserialize, Serialize)]
struct IndexArgs {
    group_id: Option<String>,
}
impl FromArgs for IndexArgs {}
pub fn index(args: Value, entry_service: &dyn EntryService) -> ApiResult {
    let args = IndexArgs::from_args(args)?;
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
impl FromArgs for CreateArgs {}
pub fn create(args: Value, entry_service: &mut dyn EntryService) -> ApiResult {
    let args = CreateArgs::from_args(args)?;

    if entry_service
        .find_by_group_id(args.group_id.as_str())
        .into_iter()
        .any(|entry| entry.meta.path == args.path.as_str())
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
        .and_then(Response::created)
}

#[derive(Deserialize, Serialize)]
struct SortArgs {
    by: MetaCmpBy,
    ascend: bool,
}
impl FromArgs for SortArgs {}
pub fn sort(args: Value, entry_service: &mut dyn EntryService) -> ApiResult {
    let args = SortArgs::from_args(args)?;
    entry_service.sort(args.by, args.ascend);
    Ok(Response::no_content())
}

pub fn show(id: &str, entry_service: &dyn EntryService) -> ApiResult {
    Response::ok(
        entry_service
            .find_by_id(id)
            .ok_or_else(Response::not_found)?,
    )
}

pub fn destroy(id: &str, entry_service: &mut dyn EntryService) -> ApiResult {
    entry_service
        .destroy(id)
        .map_err(|_| Response::not_found())?;
    Ok(Response::no_content())
}

#[derive(Deserialize, Serialize)]
struct ShiftArgs {
    offset: i64,
}
impl FromArgs for ShiftArgs {}
pub fn shift(id: &str, args: Value, entry_service: &mut dyn EntryService) -> ApiResult {
    let args = ShiftArgs::from_args(args)?;
    let mut entries = entry_service.all();
    let index = entries
        .iter()
        .position(|entry| entry.id == id)
        .ok_or_else(Response::not_found)?;
    let new_index = (index as i64 + args.offset)
        .max(0)
        .min(entries.len() as i64 - 1) as usize;
    let deleted = entries.remove(index);
    entries.insert(new_index, deleted);
    entry_service.set_all(entries);
    Ok(Response::no_content())
}
