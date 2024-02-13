use super::*;
use crate::shared::fixture_dir;
use serde_json::json;

fn playlist_default() -> Box<dyn Playlist> {
    Box::<fs::Playlist>::default()
}

fn playlist_filled() -> Box<dyn Playlist> {
    playlist_default().tap_mut(|playlist| {
        let entries = playlist.scan(
            fixture_dir().to_str().unwrap().to_string(),
            vec!["image".to_string(), "video".to_string()],
        );
        let groups = playlist
            .new_groups(vec![
                fixture_dir()
                    .join("a")
                    .join("a")
                    .to_str()
                    .unwrap()
                    .to_string(),
                fixture_dir()
                    .join("a")
                    .join("b")
                    .to_str()
                    .unwrap()
                    .to_string(),
                fixture_dir().join("b").to_str().unwrap().to_string(),
                fixture_dir().join("c").to_str().unwrap().to_string(),
            ])
            .unwrap();
        playlist.create_groups(groups);
        playlist.create_entries(entries);
    })
}

mod meta {
    use super::*;

    fn meta_1() -> Meta {
        Meta {
            path: "/1/path".to_string(),
            created_at: DateTime::<Utc>::from_timestamp_millis(2).unwrap(),
            updated_at: DateTime::<Utc>::from_timestamp_millis(3).unwrap(),
        }
    }

    fn meta_2() -> Meta {
        Meta {
            path: "/2/path".to_string(),
            created_at: DateTime::<Utc>::from_timestamp_millis(1).unwrap(),
            updated_at: DateTime::<Utc>::from_timestamp_millis(4).unwrap(),
        }
    }

    #[test]
    fn cmp_by() {
        let meta1 = meta_1();
        let meta2 = meta_2();
        assert!(meta1.cmp_by(&meta2, MetaCmpBy::Path, true).is_lt());
        assert!(meta2.cmp_by(&meta1, MetaCmpBy::Path, false).is_lt());
        assert!(meta1.cmp_by(&meta2, MetaCmpBy::CreatedAt, true).is_gt());
        assert!(meta2.cmp_by(&meta1, MetaCmpBy::CreatedAt, false).is_gt());
        assert!(meta1.cmp_by(&meta2, MetaCmpBy::UpdatedAt, true).is_lt());
        assert!(meta2.cmp_by(&meta1, MetaCmpBy::UpdatedAt, false).is_lt());
    }
}

#[test]
fn scan() {
    let playlist = playlist_default();
    let entries = playlist.scan(
        fixture_dir().to_str().unwrap().to_string(),
        vec!["image".to_string(), "video/mp4".to_string()],
    );
    assert_eq!(entries.len(), 12);
    let first = entries.first().unwrap();
    assert_eq!(first.mime, "video/mp4");
    assert_eq!(first.meta.path.split('/').last().unwrap(), "1.mp4");
}

#[test]
fn new_groups() {
    let playlist = playlist_default();
    let groups = playlist
        .new_groups(vec![
            fixture_dir()
                .join("a")
                .join("a")
                .to_str()
                .unwrap()
                .to_string(),
            fixture_dir().join("b").to_str().unwrap().to_string(),
        ])
        .unwrap();
    assert_eq!(groups.len(), 2);
    assert_eq!(
        groups.first().unwrap().meta.path.split('/').last().unwrap(),
        "a"
    );
}

#[test]
fn search() {
    let playlist = playlist_filled();
    let result = playlist.search(
        serde_json::from_value(json!({
            "mimes": ["image"],
            "path": "1",
            "order_by": "created_at",
            "ascend": false,
            "offset": 1,
            "limit": 2,
        }))
        .unwrap(),
    );
    assert_eq!(result.total, 4);
    assert_eq!(result.result.len(), 2);
}

#[test]
fn remove() {
    let mut playlist = playlist_filled();
    assert_eq!(playlist.groups().len(), 4);
    playlist.remove(&[fixture_dir()
        .join("a")
        .join("b")
        .join("1.png")
        .to_str()
        .unwrap()
        .to_string()]);
    assert_eq!(playlist.groups().len(), 3);
}
