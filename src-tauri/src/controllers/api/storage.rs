use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::services::{
    entry::EntryService, group::GroupService, playlist::PlaylistService, storage::StorageService,
};

use super::{ApiResult, FromArgs, Response};

#[derive(Deserialize, Serialize)]
struct WriteArgs {
    path: String,
}
impl FromArgs for WriteArgs {}
pub fn write(
    args: Value,
    storage_service: &dyn StorageService,
    playlist_service: &dyn PlaylistService,
    group_service: &dyn GroupService,
    entry_service: &dyn EntryService,
) -> ApiResult {
    let args = WriteArgs::from_args(args)?;
    storage_service
        .write(
            args.path.as_str(),
            playlist_service,
            group_service,
            entry_service,
        )
        .map_err(|err| Response::bad_request(err.to_string()))
        .map(|_| Response::no_content())
}

#[derive(Deserialize, Serialize)]
struct ReadArgs {
    path: String,
}
impl FromArgs for ReadArgs {}
pub fn read(
    args: Value,
    storage_service: &mut dyn StorageService,
    playlist_service: &mut dyn PlaylistService,
    group_service: &mut dyn GroupService,
    entry_service: &mut dyn EntryService,
) -> ApiResult {
    let args = ReadArgs::from_args(args)?;
    storage_service
        .read(
            args.path.as_str(),
            playlist_service,
            group_service,
            entry_service,
        )
        .map_err(|err| Response::bad_request(err.to_string()))
        .map(|_| Response::no_content())
}
