use anyhow::Result;
use chrono::{DateTime, Utc};
use fuzzy_matcher::{skim::SkimMatcherV2, FuzzyMatcher};
use serde::{Deserialize, Serialize};
use std::{cmp::Ordering, collections::HashSet};
use tap::Tap;

mod entry;
mod fs;

mod group;

fn match_mime(mime: impl AsRef<str>, patterns: impl AsRef<[String]>) -> bool {
    patterns
        .as_ref()
        .iter()
        .any(|pattern| mime.as_ref().starts_with(pattern))
}

#[derive(Clone, Copy, Deserialize)]
enum MetaCmpBy {
    #[serde(rename = "path")]
    Path,
    #[serde(rename = "created_at")]
    CreatedAt,
    #[serde(rename = "updated_at")]
    UpdatedAt,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Meta {
    path: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl Meta {
    fn cmp_by(&self, other: &Meta, by: MetaCmpBy, ascend: bool) -> Ordering {
        match by {
            MetaCmpBy::Path => {
                if ascend {
                    self.path.cmp(&other.path)
                } else {
                    other.path.cmp(&self.path)
                }
            }
            MetaCmpBy::CreatedAt => {
                if ascend {
                    self.created_at.cmp(&other.created_at)
                } else {
                    other.created_at.cmp(&self.created_at)
                }
            }
            MetaCmpBy::UpdatedAt => {
                if ascend {
                    self.updated_at.cmp(&other.updated_at)
                } else {
                    other.updated_at.cmp(&self.updated_at)
                }
            }
        }
    }
}

#[derive(Deserialize)]
pub struct SearchParams {
    mimes: Vec<String>,
    path: String,
    order_by: MetaCmpBy,
    ascend: bool,
    offset: usize,
    limit: usize,
}

#[derive(Serialize)]
pub struct SearchResult {
    total: usize,
    result: Vec<group::Group>,
}

pub trait Playlist: Send + Sync {
    fn groups(&self) -> &Vec<group::Group>;

    fn groups_mut(&mut self) -> &mut Vec<group::Group>;

    fn new_groups(&self, paths: Vec<String>) -> Result<Vec<group::Group>>;

    fn create_groups(&mut self, groups: Vec<group::Group>) {
        let existing_group_path_set = self
            .groups()
            .iter()
            .map(|group| group.meta.path.clone())
            .collect::<HashSet<_>>();
        self.groups_mut().append(
            &mut groups
                .into_iter()
                .filter(|group| !existing_group_path_set.contains(&group.meta.path))
                .collect(),
        );
    }

    fn new_entries(&self, root_path: String, allowed_mimes: Vec<String>) -> Vec<entry::Entry>;

    fn create_entries(&mut self, entries: Vec<entry::Entry>) -> Vec<entry::Entry> {
        self.groups_mut()
            .iter_mut()
            .fold(entries, |acc, cur| cur.take_matched_entries(acc))
    }

    fn delete_entries(&mut self, paths: Vec<String>) {
        let paths_set = paths.iter().collect::<HashSet<_>>();
        self.groups_mut().retain_mut(|group| {
            group
                .entries
                .retain(|entry| !paths_set.contains(&entry.meta.path));
            !group.entries.is_empty()
        })
    }

    fn search(&self, params: SearchParams) -> SearchResult {
        let matcher = SkimMatcherV2::default();
        let groups = self
            .groups()
            .iter()
            .filter_map(|group| {
                let entries = group
                    .entries
                    .iter()
                    .filter(|entry| {
                        match_mime(entry.mime.as_str(), params.mimes.as_slice())
                            && matcher
                                .fuzzy_match(entry.meta.path.as_str(), params.path.as_str())
                                .is_some()
                    })
                    .cloned()
                    .collect::<Vec<_>>();
                if entries.is_empty() {
                    None
                } else {
                    Some(group::Group {
                        meta: group.meta.clone(),
                        entries,
                    })
                }
            })
            .collect::<Vec<_>>()
            .tap_mut(|groups| {
                groups.sort_by(|a, b| a.meta.cmp_by(&b.meta, params.order_by, params.ascend))
            });
        let total = groups.len();
        let result = groups
            .into_iter()
            .skip(params.offset)
            .take(params.limit)
            .collect();
        SearchResult { total, result }
    }
}

pub fn instantiate() -> Box<dyn Playlist> {
    Box::<fs::Playlist>::default()
}

#[cfg(test)]
mod tests {
    use chrono::{DateTime, Utc};
    use serde_json::json;
    use tap::Tap;

    use super::*;
    use crate::shared::fixture_dir;

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

    fn playlist_default() -> Box<dyn Playlist> {
        instantiate()
    }

    fn playlist_filled() -> Box<dyn Playlist> {
        playlist_default().tap_mut(|playlist| {
            let entries = playlist.new_entries(
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
    fn delete_entries() {
        let mut playlist = playlist_filled();
        assert_eq!(playlist.groups().len(), 4);
        playlist.delete_entries(vec![fixture_dir()
            .join("a")
            .join("b")
            .join("1.png")
            .to_str()
            .unwrap()
            .to_string()]);
        assert_eq!(playlist.groups().len(), 3);
    }
}
