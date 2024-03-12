use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{models::playlist::Playlist, services::playlist::PlaylistService};

use super::{ApiResult, Response};

pub fn index(playlist_service: &dyn PlaylistService) -> ApiResult {
    Response::ok(playlist_service.all())
}

#[derive(Deserialize, Serialize)]
struct CreateArgs {
    name: String,
}
pub fn create(args: Value, playlist_service: &mut dyn PlaylistService) -> ApiResult {
    let args: CreateArgs =
        serde_json::from_value(args).map_err(|err| Response::bad_request(err.to_string()))?;
    let playlist = Playlist::new(args.name);
    playlist
        .save(playlist_service)
        .map_err(|err| {
            error!("Cannot save playlist: {}", err);
            Response::internal_server_error()
        })
        .and_then(|p| Response::created(p))
}

pub fn show(id: &str, playlist_service: &dyn PlaylistService) -> ApiResult {
    Response::ok(
        playlist_service
            .find_by_id(id)
            .ok_or_else(|| Response::not_found())?,
    )
}

pub fn destroy(id: &str, playlist_service: &mut dyn PlaylistService) -> ApiResult {
    playlist_service
        .destroy(id)
        .map_err(|_| Response::not_found())?;
    Ok(Response::no_content())
}
