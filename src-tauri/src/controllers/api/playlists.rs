use std::{fs::File, path::Path};

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{models::playlist::Playlist, services::playlist::PlaylistService, utils::meta::Meta};

use super::{ApiResult, Response};

pub fn index(playlist_service: &dyn PlaylistService) -> ApiResult {
    Response::ok(playlist_service.all())
}

pub fn show(id: &str, playlist_service: &dyn PlaylistService) -> ApiResult {
    Response::ok(playlist_service.find_by_id(id))
}

#[derive(Deserialize, Serialize)]
struct CreateArgs {
    path: String,
}
pub fn create(args: Value, playlist_service: &mut dyn PlaylistService) -> ApiResult {
    let args: CreateArgs =
        serde_json::from_value(args).map_err(|err| Response::bad_request(err.to_string()))?;
    let path = Path::new(args.path.as_str());
    if !path.exists() {
        File::create(path).map_err(|err| Response::bad_request(err.to_string()))?;
    }
    let meta = Meta::from_path(path).map_err(|err| Response::bad_request(err.to_string()))?;
    let playlist = Playlist::new(meta);
    playlist
        .save(playlist_service)
        .map_err(|err| {
            error!("Cannot save playlist: {}", err);
            Response::internal_server_error()
        })
        .and_then(|p| Response::created(p))
}

pub fn destroy(id: &str, playlist_service: &mut dyn PlaylistService) -> ApiResult {
    playlist_service
        .destroy(id)
        .map_err(|_| Response::not_found())?;
    Ok(Response::no_content())
}
