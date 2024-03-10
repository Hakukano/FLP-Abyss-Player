use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::{ApiResult, Response};
use crate::models::playlist::{meta::MetaCmpBy, Playlist};

pub fn index(playlist: &dyn Playlist) -> ApiResult {
    Response::ok(playlist.groups().clone())
}

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

#[derive(Deserialize, Serialize)]
struct SortGroupsArgs {
    by: MetaCmpBy,
    ascend: bool,
}

pub fn sort_groups(args: Value, playlist: &mut dyn Playlist) -> ApiResult {
    let args: SortGroupsArgs =
        serde_json::from_value(args).map_err(|err| Response::bad_request(err.to_string()))?;
    playlist.sort_groups(args.by, args.ascend);
    Response::ok(())
}

#[derive(Deserialize, Serialize)]
struct MoveGroupArgs {
    path: String,
    index: usize,
}

pub fn move_group(args: Value, playlist: &mut dyn Playlist) -> ApiResult {
    let args: MoveGroupArgs =
        serde_json::from_value(args).map_err(|err| Response::bad_request(err.to_string()))?;
    Response::ok(
        playlist
            .move_group(args.path, args.index)
            .map_err(|err| Response::bad_request(err.to_string()))?,
    )
}

pub fn delete_groups(args: Value, playlist: &mut dyn Playlist) -> ApiResult {
    playlist.delete_groups(
        serde_json::from_value(args).map_err(|err| Response::bad_request(err.to_string()))?,
    );
    Response::ok(())
}

#[derive(Deserialize, Serialize)]
struct NewEntriesArgs {
    root_path: String,
    allowed_mimes: Vec<String>,
}

pub fn new_entries(args: Value, playlist: &dyn Playlist) -> ApiResult {
    let args: NewEntriesArgs =
        serde_json::from_value(args).map_err(|err| Response::bad_request(err.to_string()))?;
    Response::ok(playlist.new_entries(args.root_path, args.allowed_mimes))
}

pub fn create_entries(args: Value, playlist: &mut dyn Playlist) -> ApiResult {
    Response::created(playlist.create_entries(
        serde_json::from_value(args).map_err(|err| Response::bad_request(err.to_string()))?,
    ))
}

#[derive(Deserialize, Serialize)]
struct SortEntriesArgs {
    owner: String,
    by: MetaCmpBy,
    ascend: bool,
}

pub fn sort_entries(args: Value, playlist: &mut dyn Playlist) -> ApiResult {
    let args: SortEntriesArgs =
        serde_json::from_value(args).map_err(|err| Response::bad_request(err.to_string()))?;
    Response::ok(
        playlist
            .sort_entries(args.owner, args.by, args.ascend)
            .map_err(|_| Response::not_found())?,
    )
}

#[derive(Deserialize, Serialize)]
struct MoveEntryArgs {
    owner: String,
    path: String,
    index: usize,
}

pub fn move_entry(args: Value, playlist: &mut dyn Playlist) -> ApiResult {
    let args: MoveEntryArgs =
        serde_json::from_value(args).map_err(|err| Response::bad_request(err.to_string()))?;
    Response::ok(
        playlist
            .move_entry(args.owner, args.path, args.index)
            .map_err(|err| Response::bad_request(err.to_string()))?,
    )
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
        models::playlist::{entry::Entry, group::Group, instantiate},
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

    fn mock_new_entries_args() -> NewEntriesArgs {
        NewEntriesArgs {
            root_path: fixtures_dir().to_str().unwrap().to_string(),
            allowed_mimes: vec!["image".to_string()],
        }
    }

    fn mock_entries() -> Vec<Entry> {
        let args = mock_new_entries_args();
        mock_playlist_default().new_entries(args.root_path, args.allowed_mimes)
    }

    fn mock_playlist_with_entries() -> Box<dyn Playlist> {
        mock_playlist_with_groups().tap_mut(|playlist| {
            playlist.create_entries(mock_entries());
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

    #[test]
    fn sort_groups() {
        let mut playlist = mock_playlist_with_groups();
        let resp = super::sort_groups(
            serde_json::to_value(SortGroupsArgs {
                by: MetaCmpBy::Path,
                ascend: false,
            })
            .unwrap(),
            playlist.as_mut(),
        )
        .unwrap();
        assert_eq!(resp.status, 200);
        assert_eq!(
            &playlist.groups().first().unwrap().meta.path,
            mock_group_paths().last().unwrap()
        );
    }

    #[test]
    fn move_group() {
        let mut playlist = mock_playlist_with_groups();
        let resp = super::move_group(
            serde_json::to_value(MoveGroupArgs {
                path: mock_group_paths().first().unwrap().to_string(),
                index: 1,
            })
            .unwrap(),
            playlist.as_mut(),
        )
        .unwrap();
        assert_eq!(resp.status, 200);
        assert_eq!(
            &playlist.groups().first().unwrap().meta.path,
            mock_group_paths().get(1).unwrap()
        );
    }

    #[test]
    fn delete_groups() {
        let mut playlist = mock_playlist_with_groups();
        let resp = super::delete_groups(
            serde_json::to_value(vec![
                mock_group_paths().first().unwrap().to_string(),
                mock_group_paths().get(1).unwrap().to_string(),
            ])
            .unwrap(),
            playlist.as_mut(),
        )
        .unwrap();
        assert_eq!(resp.status, 200);
        assert_eq!(
            &playlist.groups().first().unwrap().meta.path,
            mock_group_paths().get(2).unwrap()
        );
    }

    #[test]
    fn new_entries() {
        let playlist = mock_playlist_default();
        let body = super::new_entries(
            serde_json::to_value(mock_new_entries_args()).unwrap(),
            playlist.as_ref(),
        )
        .unwrap()
        .body;
        let entries = body.as_array().unwrap();
        assert_eq!(
            entries
                .first()
                .unwrap()
                .get("meta")
                .unwrap()
                .get("path")
                .unwrap()
                .as_str()
                .unwrap(),
            mock_group_paths().first().unwrap().clone() + "/1.png"
        );
    }

    #[test]
    fn create_entries() {
        let mut playlist = mock_playlist_with_groups();
        let resp = super::create_entries(
            serde_json::to_value(mock_entries()).unwrap(),
            playlist.as_mut(),
        )
        .unwrap();
        assert_eq!(resp.status, 201);
        assert_eq!(
            playlist
                .groups()
                .first()
                .unwrap()
                .entries
                .first()
                .unwrap()
                .meta
                .path,
            mock_group_paths().first().unwrap().clone() + "/1.png"
        );
    }

    #[test]
    fn sort_entries() {
        let mut playlist = mock_playlist_with_entries();
        let origin_last = playlist
            .groups()
            .last()
            .unwrap()
            .entries
            .last()
            .unwrap()
            .meta
            .path
            .clone();
        let resp = super::sort_entries(
            serde_json::to_value(SortEntriesArgs {
                owner: mock_group_paths().last().unwrap().to_string(),
                by: MetaCmpBy::Path,
                ascend: true,
            })
            .unwrap(),
            playlist.as_mut(),
        )
        .unwrap();
        assert_eq!(resp.status, 200);
        assert_eq!(
            playlist
                .groups()
                .last()
                .unwrap()
                .entries
                .first()
                .unwrap()
                .meta
                .path,
            origin_last
        );
    }

    #[test]
    fn move_entry() {
        let mut playlist = mock_playlist_with_entries();
        let origin_last = playlist
            .groups()
            .last()
            .unwrap()
            .entries
            .last()
            .unwrap()
            .meta
            .path
            .clone();
        let resp = super::move_entry(
            serde_json::to_value(MoveEntryArgs {
                owner: mock_group_paths().last().unwrap().to_string(),
                path: origin_last.clone(),
                index: 0,
            })
            .unwrap(),
            playlist.as_mut(),
        )
        .unwrap();
        assert_eq!(resp.status, 200);
        assert_eq!(
            playlist
                .groups()
                .last()
                .unwrap()
                .entries
                .first()
                .unwrap()
                .meta
                .path,
            origin_last
        );
    }

    #[test]
    fn delete_entries() {
        let mut playlist = mock_playlist_with_entries();
        let before_delete = playlist.groups().len();
        let resp = super::delete_entries(
            json!([mock_group_paths().first().unwrap().clone() + "/1.png"]),
            playlist.as_mut(),
        )
        .unwrap();
        assert_eq!(resp.status, 204);
        assert_eq!(playlist.groups().len(), before_delete - 1);
    }

    #[test]
    fn search() {
        let playlist = mock_playlist_with_entries();
        let body = super::search(
            json!({
                "mimes": ["image"],
                "path": "b/2",
                "offset": 0,
                "limit": 10
            }),
            playlist.as_ref(),
        )
        .unwrap()
        .body;
        assert_eq!(body.get("total").unwrap(), &json!(1));
        assert_eq!(body.get("results").unwrap().as_array().unwrap().len(), 1);
    }
}
