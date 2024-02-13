use anyhow::Result;
use chrono::{DateTime, Utc};
use fuzzy_matcher::{skim::SkimMatcherV2, FuzzyMatcher};
use serde::{Deserialize, Serialize};
use std::{cmp::Ordering, collections::HashSet};
use tap::Tap;

mod entry;
mod fs;

mod group;

#[cfg(test)]
mod tests;

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

#[derive(Clone, Debug, Serialize)]
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

pub trait Playlist {
    fn scan(&self, root_path: String, allowed_mimes: Vec<String>) -> Vec<entry::Entry>;

    fn new_groups(&self, paths: Vec<String>) -> Result<Vec<group::Group>>;

    fn groups(&self) -> &Vec<group::Group>;

    fn groups_mut(&mut self) -> &mut Vec<group::Group>;

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

    fn create_entries(&mut self, entries: Vec<entry::Entry>) -> Vec<entry::Entry> {
        self.groups_mut()
            .iter_mut()
            .fold(entries, |acc, cur| cur.take_matched_entries(acc))
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

    fn remove(&mut self, paths: &[String]) {
        let paths_set = paths.iter().collect::<HashSet<_>>();
        self.groups_mut().retain_mut(|group| {
            group
                .entries
                .retain(|entry| !paths_set.contains(&entry.meta.path));
            !group.entries.is_empty()
        })
    }
}

pub fn instantiate() -> Box<dyn Playlist> {
    Box::<fs::Playlist>::default()
}
