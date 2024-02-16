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

#[derive(Deserialize)]
struct ScanArgs {
    root_path: String,
    allowed_mimes: Vec<String>,
}

pub fn new_entries(args: Value, playlist: &dyn Playlist) -> ApiResult {
    let args: ScanArgs =
        serde_json::from_value(args).map_err(|err| Response::bad_request(err.to_string()))?;
    Response::ok(playlist.new_entries(args.root_path, args.allowed_mimes))
}

pub fn create_entries(args: Value, playlist: &mut dyn Playlist) -> ApiResult {
    Response::created(playlist.create_entries(
        serde_json::from_value(args).map_err(|err| Response::bad_request(err.to_string()))?,
    ))
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
    use tap::Tap;

    use super::*;
    use crate::{
        models::playlist::{group::Group, instantiate},
        shared::test::fixtures_dir,
    };

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

    fn mock_groups() -> Vec<Group> {
        mock_playlist_default()
            .new_groups(mock_group_paths())
            .unwrap()
    }

    fn mock_playlist_with_groups() -> Box<dyn Playlist> {
        mock_playlist_default().tap_mut(|playlist| {
            playlist.create_groups(mock_groups());
        })
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

    #[test]
    fn create_groups() {
        let mut playlist = mock_playlist_default();
        let resp = super::create_groups(
            serde_json::to_value(mock_groups()).unwrap(),
            playlist.as_mut(),
        )
        .unwrap();
        assert_eq!(resp.status, 201);
        assert_eq!(
            &playlist.groups().first().unwrap().meta.path,
            mock_group_paths().first().unwrap()
        );
    }
}
