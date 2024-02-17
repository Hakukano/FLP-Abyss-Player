use crate::models::playlist::meta::MetaCmpBy;
use anyhow::{anyhow, Result};
use fuzzy_matcher::{skim::SkimMatcherV2, FuzzyMatcher};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

pub mod entry;
mod fs;

pub mod group;
pub mod meta;

fn match_mime(mime: impl AsRef<str>, patterns: impl AsRef<[String]>) -> bool {
    patterns
        .as_ref()
        .iter()
        .any(|pattern| mime.as_ref().starts_with(pattern))
}

#[derive(Deserialize)]
pub struct SearchParams {
    mimes: Vec<String>,
    path: String,
    offset: usize,
    limit: usize,
}

#[derive(Serialize)]
pub struct SearchResult {
    total: usize,
    results: Vec<group::Group>,
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

    fn sort_group(&mut self, by: MetaCmpBy, ascend: bool) {
        self.groups_mut()
            .sort_by(|a, b| a.meta.cmp_by(&b.meta, by, ascend));
    }

    fn move_group(&mut self, path: String, index: usize) -> Result<()> {
        if index >= self.groups().len() {
            return Err(anyhow!("Out of range"));
        }
        let origin = self
            .groups()
            .iter()
            .position(|group| group.meta.path == path)
            .ok_or_else(|| anyhow!("Group not found"))?;
        let group = self.groups_mut().remove(origin);
        self.groups_mut().insert(index, group);
        Ok(())
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
            .collect::<Vec<_>>();
        let total = groups.len();
        let results = groups
            .into_iter()
            .skip(params.offset)
            .take(params.limit)
            .collect();
        SearchResult { total, results }
    }
}

pub fn instantiate() -> Box<dyn Playlist> {
    Box::<fs::Playlist>::default()
}

#[cfg(test)]
mod tests {
    use serde_json::json;
    use tap::Tap;

    use super::*;
    use crate::shared::test::fixtures_dir;

    fn playlist_default() -> Box<dyn Playlist> {
        instantiate()
    }

    fn playlist_filled() -> Box<dyn Playlist> {
        playlist_default().tap_mut(|playlist| {
            let entries = playlist.new_entries(
                fixtures_dir().to_str().unwrap().to_string(),
                vec!["image".to_string(), "video".to_string()],
            );
            let groups = playlist
                .new_groups(vec![
                    fixtures_dir()
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
                    fixtures_dir().join("b").to_str().unwrap().to_string(),
                    fixtures_dir().join("c").to_str().unwrap().to_string(),
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
                "offset": 1,
                "limit": 2,
            }))
            .unwrap(),
        );
        assert_eq!(result.total, 4);
        assert_eq!(result.results.len(), 2);
    }

    #[test]
    fn delete_entries() {
        let mut playlist = playlist_filled();
        assert_eq!(playlist.groups().len(), 4);
        playlist.delete_entries(vec![fixtures_dir()
            .join("a")
            .join("b")
            .join("1.png")
            .to_str()
            .unwrap()
            .to_string()]);
        assert_eq!(playlist.groups().len(), 3);
    }
}
