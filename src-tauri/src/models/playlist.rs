use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::{
    cmp::Ordering,
    path::{Path, PathBuf},
};

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
    search: String,
    order_by: MetaCmpBy,
    ascend: bool,
    limit: usize,
    offset: usize,
}

#[derive(Serialize)]
pub struct SearchGroupsResult {
    total: usize,
    result: Vec<group::Group>,
}

#[derive(Deserialize)]
pub struct SearchEntriesParams {
    search: String,
    mimes: Vec<String>,
    order_by: MetaCmpBy,
    ascend: bool,
    limit: usize,
    offset: usize,
}

#[derive(Serialize)]
pub struct SearchEntriesResult {
    total: usize,
    result: Vec<entry::Entry>,
}

pub trait Playlist {
    fn scan(&mut self, path: impl AsRef<Path>, allowed_mimes: impl AsRef<[String]>) -> Result<()>;

    fn groups(&self) -> &[group::Group];

    fn search_groups(&self, params: SearchGroupsParams) -> Result<SearchGroupsResult>;

    fn remove(&mut self, paths: impl AsRef<[String]>) -> Result<()>;
}
