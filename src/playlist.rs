use std::{
    fmt::Display,
    fs::File,
    io::{BufWriter, Read, Write},
    path::{Path, PathBuf},
};

use anyhow::{anyhow, Result};
use chrono::{DateTime, Utc};

use crate::{
    config::{Config, MediaType, VideoPlayer},
    VERSION_MAJOR, VERSION_MINOR, VERSION_PATCH,
};

mod parser;
mod writer;

pub const EXTENSION: &str = "fappl";

const FLP: &[u8] = b"FLP";
const APPL: &[u8] = b"APPL";

#[derive(Clone)]
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

#[derive(Clone)]
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

    pub fn writer_config(&self, config: &mut Config) {
        config.media_type = self.media_type;
        config.video_player = self.video_player;
        config.video_player_path = self.video_player_path.clone();
    }

    pub fn load(path: impl AsRef<Path>) -> Result<(Vec<u8>, Self)> {
        let mut bytes = Vec::new();
        File::open(path)?.read_to_end(&mut bytes)?;
        let (bytes, header) =
            parser::header(bytes.as_slice()).map_err(|err| anyhow!(err.to_string()))?;
        Ok((bytes.to_vec(), header))
    }

    pub fn save(&self) -> Vec<u8> {
        writer::header(self)
    }
}

pub struct Body {
    item_paths: Vec<String>,
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

    pub fn write_paths(&self, paths: &mut Vec<PathBuf>) {
        *paths = self.item_paths.iter().map(|p| p.into()).collect();
    }

    pub fn load(data: impl AsRef<[u8]>) -> Result<Self> {
        Ok(parser::body(data.as_ref())
            .map_err(|err| anyhow!(err.to_string()))?
            .1)
    }

    pub fn save(&self, buffer: Vec<u8>, path: impl AsRef<Path>) -> Result<()> {
        let bytes = writer::body(buffer, self);
        BufWriter::new(File::create(path)?).write_all(bytes.as_slice())?;
        Ok(())
    }
}
