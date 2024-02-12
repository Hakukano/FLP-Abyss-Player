use anyhow::Result;
use chrono::{DateTime, Utc};
use fuzzy_matcher::{skim::SkimMatcherV2, FuzzyMatcher};
use serde::{Deserialize, Serialize};
use std::{cmp::Ordering, collections::HashSet, path::Path};
use tap::Tap;

pub mod entry;
mod fs;
pub mod group;

#[derive(Clone, Copy, Deserialize)]
enum MetaCmpBy {
    #[serde(rename = "path")]
    Path,
    #[serde(rename = "created_at")]
    CreatedAt,
    #[serde(rename = "updated_at")]
    UpdatedAt,
}

#[derive(Clone, Serialize)]
struct Meta {
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
    path_pattern: String,
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
    fn scan(&self, root_path: String, allowed_mimes: Vec<String>) -> Result<Vec<entry::Entry>>;

    fn create_groups(&mut self, paths: Vec<String>) -> Result<()>;

    fn groups(&self) -> &Vec<group::Group>;

    fn groups_mut(&mut self) -> &mut Vec<group::Group>;

    fn create_entries(&mut self, entries: Vec<entry::Entry>) -> Vec<entry::Entry> {
        self.groups_mut()
            .iter_mut()
            .fold(entries, |acc, cur| cur.take_matched_entries(acc))
    }

    fn search_groups(&self, params: SearchParams) -> SearchResult {
        let mimes_set = params.mimes.iter().collect::<HashSet<_>>();
        let matcher = SkimMatcherV2::default();
        let groups = self
            .groups()
            .iter()
            .filter_map(|group| {
                let entries = group
                    .entries
                    .iter()
                    .filter(|entry| {
                        mimes_set.contains(&entry.mime)
                            && matcher
                                .fuzzy_match(entry.meta.path.as_str(), params.path_pattern.as_str())
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

    fn remove(&mut self, paths: impl AsRef<[String]>) {
        let paths_set = paths.as_ref().iter().collect::<HashSet<_>>();
        self.groups_mut().retain_mut(|group| {
            group
                .entries
                .retain(|entry| !paths_set.contains(&entry.meta.path));
            !group.entries.is_empty()
        })
    }
}
