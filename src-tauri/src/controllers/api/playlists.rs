use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{models::playlist::Playlist, services::playlist::PlaylistService};

use super::{ApiResult, FromArgs, Response};

pub fn index(playlist_service: &dyn PlaylistService) -> ApiResult {
    Response::ok(playlist_service.all())
}

#[derive(Deserialize, Serialize)]
struct CreateArgs {
    name: String,
}
impl FromArgs for CreateArgs {}
pub fn create(args: Value, playlist_service: &mut dyn PlaylistService) -> ApiResult {
    let args = CreateArgs::from_args(args)?;
    let playlist = Playlist::new(args.name);
    playlist_service
        .save(playlist)
        .map_err(|err| {
            error!("Cannot save playlist: {}", err);
            Response::internal_server_error()
        })
        .and_then(Response::created)
}

pub fn show(id: &str, playlist_service: &dyn PlaylistService) -> ApiResult {
    Response::ok(
        playlist_service
            .find_by_id(id)
            .ok_or_else(Response::not_found)?,
    )
}

pub fn destroy(id: &str, playlist_service: &mut dyn PlaylistService) -> ApiResult {
    playlist_service
        .destroy(id)
        .map_err(|_| Response::not_found())?;
    Ok(Response::no_content())
}
