use crate::services::playlist::PlaylistService;

use super::{ApiResult, Response};

pub fn index(playlist_service: &dyn PlaylistService) -> ApiResult {
    Response::ok(playlist_service.all())
}

pub fn show(id: &str, playlist_service: &dyn PlaylistService) -> ApiResult {
    Response::ok(playlist_service.find_by_id(id))
}

pub fn destroy(id: &str, playlist_service: &mut dyn PlaylistService) -> ApiResult {
    playlist_service
        .destroy(id)
        .map_err(|_| Response::not_found())?;
    Ok(Response::no_content())
}
