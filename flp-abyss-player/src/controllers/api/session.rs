use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::services::{
    entry::EntryService, group::GroupService, playlist::PlaylistService, session::SessionService,
};

use super::{ApiResult, FromArgs, Response};

#[derive(Deserialize, Serialize)]
struct WriteArgs {
    path: String,
}
impl FromArgs for WriteArgs {}
pub fn save(
    args: Value,
    session_service: &dyn SessionService,
    playlist_service: &dyn PlaylistService,
    group_service: &dyn GroupService,
    entry_service: &dyn EntryService,
) -> ApiResult {
    let args = WriteArgs::from_args(args)?;
    session_service
        .save(
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
pub fn load(
    args: Value,
    session_service: &mut dyn SessionService,
    playlist_service: &mut dyn PlaylistService,
    group_service: &mut dyn GroupService,
    entry_service: &mut dyn EntryService,
) -> ApiResult {
    let args = ReadArgs::from_args(args)?;
    session_service
        .load(
            args.path.as_str(),
            playlist_service,
            group_service,
            entry_service,
        )
        .map_err(|err| Response::bad_request(err.to_string()))
        .map(|_| Response::no_content())
}
