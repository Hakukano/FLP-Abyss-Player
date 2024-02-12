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
pub struct SearchGroupsParams {
    path_pattern: String,
    order_by: MetaCmpBy,
    ascend: bool,
    offset: usize,
    limit: usize,
}

#[derive(Serialize)]
pub struct SearchGroupsResult {
    total: usize,
    result: Vec<group::Group>,
}

#[derive(Deserialize)]
pub struct SearchEntriesParams {
    groups: Vec<String>,
    mimes: Vec<String>,
    path_pattern: String,
    order_by: MetaCmpBy,
    ascend: bool,
    offset: usize,
    limit: usize,
}

#[derive(Serialize)]
pub struct SearchEntriesResult {
    total: usize,
    result: Vec<entry::Entry>,
}

pub trait Playlist {
    fn scan(&mut self, path: impl AsRef<Path>, allowed_mimes: impl AsRef<[String]>) -> Result<()>;

    fn groups(&self) -> &[group::Group];

    fn remove(&mut self, paths: impl AsRef<[String]>) -> Result<()>;

    fn search_groups(&self, params: SearchGroupsParams) -> SearchGroupsResult {
        let matcher = SkimMatcherV2::default();
        let groups = self
            .groups()
            .iter()
            .filter(|group| {
                matcher
                    .fuzzy_match(group.meta.path.as_str(), params.path_pattern.as_str())
                    .is_some()
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
            .cloned()
            .collect();
        SearchGroupsResult { total, result }
    }

    fn search_entries(&self, params: SearchEntriesParams) -> SearchEntriesResult {
        let groups_set: HashSet<String> = params.groups.into_iter().collect();
        let mimes_set: HashSet<String> = params.mimes.into_iter().collect();
        let matcher = SkimMatcherV2::default();
        let entries = self
            .groups()
            .iter()
            .filter(|group| groups_set.contains(&group.meta.path))
            .fold(Vec::new(), |acc, cur| {
                acc.tap_mut(|a| cur.entries.iter().for_each(|entry| a.push(entry)))
            })
            .into_iter()
            .filter(|entry| {
                mimes_set.contains(&entry.mime)
                    && matcher
                        .fuzzy_match(entry.meta.path.as_str(), params.path_pattern.as_str())
                        .is_some()
            })
            .collect::<Vec<_>>()
            .tap_mut(|entries| {
                entries.sort_by(|a, b| a.meta.cmp_by(&b.meta, params.order_by, params.ascend))
            });
        let total = entries.len();
        let result = entries
            .into_iter()
            .skip(params.offset)
            .take(params.limit)
            .cloned()
            .collect();
        SearchEntriesResult { total, result }
    }
}
