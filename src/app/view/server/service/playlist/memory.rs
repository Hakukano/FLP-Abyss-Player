use std::{
    path::PathBuf,
    sync::{Arc, RwLock},
};

use anyhow::{anyhow, Result};
use async_trait::async_trait;
use fuzzy_matcher::{skim::SkimMatcherV2, FuzzyMatcher};

use super::{list, read, DataBuilder};

pub struct Playlist {
    pub paths: Arc<RwLock<Vec<PathBuf>>>,
}

#[async_trait]
impl super::Playlist for Playlist {
    async fn read(&self, req: read::Request) -> Result<read::Response> {
        Ok(read::Response::builder()
            .id(req.id)
            .path(
                self.paths
                    .read()
                    .unwrap()
                    .get(req.id as usize)
                    .ok_or_else(|| anyhow!("Path not found"))?
                    .display()
                    .to_string(),
            )
            .build()?)
    }

    async fn list(&self, req: list::Request) -> Result<list::Response> {
        let matcher = SkimMatcherV2::default();
        let data = self
            .paths
            .read()
            .unwrap()
            .iter()
            .enumerate()
            .filter_map(|(i, p)| {
                let p = p.display().to_string();
                if req.search.is_empty()
                    || matcher
                        .fuzzy_match(p.as_str(), req.search.as_str())
                        .is_some()
                {
                    DataBuilder::default().id(i as u32).path(p).build().ok()
                } else {
                    None
                }
            })
            .skip(req.offset as usize)
            .take(req.length as usize)
            .collect();
        Ok(list::Response::builder()
            .data(data)
            .count(self.paths.read().unwrap().len() as u32)
            .build()?)
    }
}
