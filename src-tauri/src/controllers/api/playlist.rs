use serde::Deserialize;
use serde_json::Value;

use super::{ApiResult, Response};
use crate::models::playlist::Playlist;

pub fn new_groups(args: Value, playlist: &dyn Playlist) -> ApiResult {
    Response::ok(
        playlist
            .new_groups(
                serde_json::from_value(args)
                    .map_err(|err| Response::bad_request(err.to_string()))?,
            )
            .map_err(|err| {
                error!("Cannot new groups: {}", err);
                Response::internal_server_error()
            })?,
    )
}

pub fn create_groups(args: Value, playlist: &mut dyn Playlist) -> ApiResult {
    playlist.create_groups(
        serde_json::from_value(args).map_err(|err| Response::bad_request(err.to_string()))?,
    );
    Response::created(())
}

pub fn entries(args: Value, playlist: &dyn Playlist) -> ApiResult {
    let args: ScanArgs =
        serde_json::from_value(args).map_err(|err| Response::bad_request(err.to_string()))?;
    Response::ok(playlist.new_entries(args.root_path, args.allowed_mimes))
}

pub fn create_entries(args: Value, playlist: &mut dyn Playlist) -> ApiResult {
    Response::created(playlist.create_entries(
        serde_json::from_value(args).map_err(|err| Response::bad_request(err.to_string()))?,
    ))
}

#[derive(Deserialize)]
struct ScanArgs {
    root_path: String,
    allowed_mimes: Vec<String>,
}

pub fn delete_entries(args: Value, playlist: &mut dyn Playlist) -> ApiResult {
    playlist.delete_entries(
        serde_json::from_value(args).map_err(|err| Response::bad_request(err.to_string()))?,
    );
    Ok(Response::no_content())
}

pub fn search(args: Value, playlist: &dyn Playlist) -> ApiResult {
    Response::ok(playlist.search(
        serde_json::from_value(args).map_err(|err| Response::bad_request(err.to_string()))?,
    ))
}

#[cfg(test)]
mod test {
    use serde_json::json;

    use super::*;
    use crate::{models::playlist::instantiate, shared::test::fixtures_dir};

    fn mock_playlist_default() -> Box<dyn Playlist> {
        instantiate()
    }

    fn mock_group_paths() -> Vec<String> {
        vec![
            fixtures_dir()
                .join("a")
                .join("a")
                .join("a")
                .to_str()
                .unwrap()
                .to_string(),
            fixtures_dir()
                .join("a")
                .join("b")
                .to_str()
                .unwrap()
                .to_string(),
            fixtures_dir()
                .join("b")
                .join("a")
                .join("a")
                .to_str()
                .unwrap()
                .to_string(),
            fixtures_dir()
                .join("b")
                .join("a")
                .join("b")
                .to_str()
                .unwrap()
                .to_string(),
            fixtures_dir().join("c").to_str().unwrap().to_string(),
        ]
    }

    #[test]
    fn new_groups() {
        let playlist = mock_playlist_default();
        let body = super::new_groups(json!(mock_group_paths()), playlist.as_ref())
            .unwrap()
            .body;
        let groups = body.as_array().unwrap();
        assert_eq!(
            groups
                .first()
                .unwrap()
                .get("meta")
                .unwrap()
                .get("path")
                .unwrap(),
            mock_group_paths().first().unwrap()
        )
    }
}
