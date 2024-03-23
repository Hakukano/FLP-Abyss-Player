use std::{
    collections::HashMap,
    fmt::Display,
    fs::File,
    io::{BufWriter, Read, Write},
    path::{Path, PathBuf},
};

use anyhow::{anyhow, Result};
use chrono::{DateTime, Utc};
use flp_abyss_player_derive::Differ;
use fuzzy_matcher::{skim::SkimMatcherV2, FuzzyMatcher};
use once_cell::sync::Lazy;
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};

use crate::{
    models::config::{Config, MediaType, VideoPlayer},
    VERSION_MAJOR, VERSION_MINOR, VERSION_PATCH,
};

use super::Singleton;

mod parser;
mod writer;

pub const EXTENSION: &str = "fappl";

const FLP: &[u8] = b"FLP";
const APPL: &[u8] = b"APPL";

#[derive(Clone, Default, Deserialize, Serialize, Differ)]
pub struct Version {
    major: u8,
    minor: u8,
    patch: u8,
}

impl Version {
    fn new() -> Self {
        Self {
            major: VERSION_MAJOR.parse().unwrap(),
            minor: VERSION_MINOR.parse().unwrap(),
            patch: VERSION_PATCH.parse().unwrap(),
        }
    }

    fn is_supported(&self) -> bool {
        self.major.to_string() == VERSION_MAJOR
    }
}

impl Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

#[derive(Clone, Default, Deserialize, Serialize, Differ)]
pub struct Header {
    pub version: Version,
    pub time: DateTime<Utc>,
    pub media_type: MediaType,
    pub video_player: VideoPlayer,
    pub video_player_path: Option<String>,
}

impl Header {
    pub fn from_config(config: &Config) -> Self {
        Self {
            version: Version::new(),
            time: Utc::now(),
            media_type: config.media_type,
            video_player: config.video_player,
            video_player_path: config.video_player_path.clone(),
        }
    }

    pub fn read(path: impl AsRef<Path>) -> Result<(Vec<u8>, Self)> {
        let mut bytes = Vec::new();
        File::open(path)?.read_to_end(&mut bytes)?;
        let (bytes, header) =
            parser::header(bytes.as_slice()).map_err(|err| anyhow!(err.to_string()))?;
        Ok((bytes.to_vec(), header))
    }

    pub fn write(&self) -> Vec<u8> {
        writer::header(self)
    }
}

#[derive(Clone, Default, Deserialize, Serialize, Differ)]
pub struct Body {
    pub item_paths: Vec<String>,
}

impl Body {
    pub fn from_paths(paths: &[PathBuf]) -> Self {
        Self {
            item_paths: paths
                .iter()
                .map(|p| p.to_str().expect("Invalid path").to_string())
                .collect(),
        }
    }

    pub fn read(data: impl AsRef<[u8]>) -> Result<Self> {
        Ok(parser::body(data.as_ref())
            .map_err(|err| anyhow!(err.to_string()))?
            .1)
    }

    pub fn write(&self, buffer: Vec<u8>, path: impl AsRef<Path>) -> Result<()> {
        let bytes = writer::body(buffer, self);
        BufWriter::new(File::create(path)?).write_all(bytes.as_slice())?;
        Ok(())
    }
}

#[derive(Clone, Default, StaticRecord, Deserialize, Serialize, Differ)]
#[static_record(singleton = SINGLETON, belongs_to = ["config"])]
pub struct Playlist {
    pub id: String,
    pub header: Header,
    pub body: Body,
    pub config_id: String,
}

impl Playlist {
    pub fn new(id: String, config: &Config) -> Self {
        Self {
            id,
            header: Header::from_config(config),
            body: Body::from_paths(config.find_all_paths().as_slice()),
            config_id: config.id.clone(),
        }
    }

    pub fn item_paths(&self) -> &[String] {
        self.body.item_paths.as_slice()
    }

    pub fn read(&mut self, path: impl AsRef<Path>) -> Result<()> {
        let (rest, header) = Header::read(path)?;
        self.header = header;
        self.body = Body::read(rest)?;
        Ok(())
    }

    pub fn write(&self, path: impl AsRef<Path>) -> Result<()> {
        let buffer = self.header.write();
        self.body.write(buffer, path)
    }

    pub fn filter(&self, search_str: impl AsRef<str>) -> Vec<(usize, String)> {
        let matcher = SkimMatcherV2::default();
        self.body
            .item_paths
            .iter()
            .enumerate()
            .filter_map(|(i, p)| {
                matcher
                    .fuzzy_match(p.as_str(), search_str.as_ref())
                    .map(|_| (i, p.clone()))
            })
            .collect()
    }
}

static SINGLETON: Singleton<Playlist> = Lazy::new(RwLock::default);
