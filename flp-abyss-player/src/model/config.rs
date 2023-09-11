mod args;
mod json;

use std::{ops::RangeInclusive, sync::RwLock};

use anyhow::{anyhow, Result};
use clap::ValueEnum;
use flp_abyss_player_derive::AccessibleModel;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

use crate::CLI;

pub const AUTO_INTERVAL_RANGE: RangeInclusive<u32> = 1..=60;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize, ValueEnum)]
pub enum MediaType {
    Unset,
    Server,
    Image,
    Video,
}

impl MediaType {
    pub fn is_unset(&self) -> bool {
        matches!(self, Self::Unset)
    }
}

impl Default for MediaType {
    fn default() -> Self {
        Self::Unset
    }
}

impl ToString for MediaType {
    fn to_string(&self) -> String {
        match self {
            Self::Unset => "--".to_string(),
            Self::Server => t!("ui.config.media_type.server"),
            Self::Image => t!("ui.config.media_type.image"),
            Self::Video => t!("ui.config.media_type.video"),
        }
    }
}

impl From<u8> for MediaType {
    fn from(n: u8) -> Self {
        match n {
            255 => Self::Server,
            1 => Self::Image,
            2 => Self::Video,
            _ => Self::Unset,
        }
    }
}

impl From<MediaType> for u8 {
    fn from(media_type: MediaType) -> Self {
        match media_type {
            MediaType::Server => 255,
            MediaType::Image => 1,
            MediaType::Video => 2,
            _ => 0,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize, ValueEnum)]
pub enum VideoPlayer {
    Unset,
    #[cfg(feature = "native")]
    Native,
    Vlc,
}

impl VideoPlayer {
    pub fn is_unset(&self) -> bool {
        matches!(self, Self::Unset)
    }
}

impl Default for VideoPlayer {
    fn default() -> Self {
        Self::Unset
    }
}

impl ToString for VideoPlayer {
    fn to_string(&self) -> String {
        match self {
            Self::Unset => "--".to_string(),
            #[cfg(feature = "native")]
            Self::Native => t!("ui.config.video_player.native"),
            Self::Vlc => t!("ui.config.video_player.vlc"),
        }
    }
}

impl From<u8> for VideoPlayer {
    fn from(n: u8) -> Self {
        match n {
            #[cfg(feature = "native")]
            255 => Self::Native,
            1 => Self::Vlc,
            _ => Self::Unset,
        }
    }
}

impl From<VideoPlayer> for u8 {
    fn from(video_player: VideoPlayer) -> Self {
        match video_player {
            #[cfg(feature = "native")]
            VideoPlayer::Native => 255,
            VideoPlayer::Vlc => 1,
            _ => 0,
        }
    }
}

#[derive(Clone, Default, Deserialize, Serialize, AccessibleModel)]
#[accessible_model(singleton = CONFIG, rw_lock)]
pub struct Config {
    pub locale: String,

    pub repeat: bool,
    pub auto: bool,
    pub auto_interval: u32,
    #[serde(rename = "loop")]
    pub lop: bool,
    pub random: bool,

    pub playlist_path: Option<String>,

    pub media_type: MediaType,
    pub root_path: Option<String>,
    pub video_player: VideoPlayer,
    pub video_player_path: Option<String>,
}

impl Config {
    pub fn validate(self) -> Result<Self> {
        if !AUTO_INTERVAL_RANGE.contains(&self.auto_interval) {
            return Err(anyhow!(format!(
                "auto_interval should be in range {:?} but found {}",
                AUTO_INTERVAL_RANGE, self.auto_interval
            )));
        }

        Ok(self)
    }
}

static CONFIG: Lazy<RwLock<Config>> = Lazy::new(|| {
    let config = match (
        CLI.playlist_path.as_ref(),
        CLI.config_file.as_ref(),
        CLI.media_type,
        CLI.root_path.as_ref(),
        CLI.video_player,
    ) {
        (None, Some(config_file), _, _, _) => json::new(config_file),
        _ => args::new(),
    };
    RwLock::new(config)
});
