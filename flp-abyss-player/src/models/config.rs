pub mod args;
pub mod json;

#[cfg(test)]
pub mod tests;

use std::{
    collections::HashMap,
    ffi::OsStr,
    fmt::Display,
    ops::RangeInclusive,
    path::{Path, PathBuf},
};

use clap::ValueEnum;
use once_cell::sync::Lazy;
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};

use crate::{impl_differ_simple, utils::differ::Differ};

use super::Singleton;

pub const AUTO_INTERVAL_RANGE: RangeInclusive<u32> = 1..=60;

trait FileExtension {
    fn has_extension<S: AsRef<str>>(&self, extensions: &[S]) -> bool;
}

impl<P> FileExtension for P
where
    P: AsRef<Path>,
{
    fn has_extension<S: AsRef<str>>(&self, extensions: &[S]) -> bool {
        if let Some(extension) = self.as_ref().extension().and_then(OsStr::to_str) {
            return extensions
                .iter()
                .any(|x| x.as_ref().eq_ignore_ascii_case(extension));
        }

        false
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize, Serialize, ValueEnum)]
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

    pub fn supported_extensions(&self) -> &[&str] {
        match self {
            MediaType::Image => &["bmp", "gif", "jpeg", "jpg", "png"],
            MediaType::Server => &[
                "bmp", "gif", "jpeg", "jpg", "png", "avi", "mp4", "webm", "mp3", "wav",
            ],
            MediaType::Video => &["avi", "mkv", "mov", "mp4", "webm"],
            _ => &[],
        }
    }

    pub fn find_all_paths(&self, path: impl AsRef<Path>) -> Vec<PathBuf> {
        let mut paths = Vec::new();
        for entry in walkdir::WalkDir::new(path)
            .into_iter()
            .filter_entry(|e| {
                e.file_name()
                    .to_str()
                    .map(|s| !s.starts_with('.'))
                    .unwrap_or(false)
            })
            .filter_map(|e| e.ok())
        {
            if entry.path().has_extension(self.supported_extensions()) {
                paths.push(entry.path().to_path_buf());
            }
        }
        paths
    }
}

impl Default for MediaType {
    fn default() -> Self {
        Self::Unset
    }
}

impl Display for MediaType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Self::Unset => "--",
            Self::Server => t!("ui.config.media_type.server").as_ref(),
            Self::Image => t!("ui.config.media_type.image").as_ref(),
            Self::Video => t!("ui.config.media_type.video").as_ref(),
        };
        write!(f, "{}", str)
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

impl_differ_simple!(MediaType);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize, Serialize, ValueEnum)]
pub enum VideoPlayer {
    Unset,
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

impl Display for VideoPlayer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Self::Unset => "--",
            Self::Native => t!("ui.config.video_player.native").as_ref(),
            Self::Vlc => t!("ui.config.video_player.vlc").as_ref(),
        };
        write!(f, "{}", str)
    }
}

impl From<u8> for VideoPlayer {
    fn from(n: u8) -> Self {
        match n {
            255 => Self::Native,
            1 => Self::Vlc,
            _ => Self::Unset,
        }
    }
}

impl From<VideoPlayer> for u8 {
    fn from(video_player: VideoPlayer) -> Self {
        match video_player {
            VideoPlayer::Native => 255,
            VideoPlayer::Vlc => 1,
            _ => 0,
        }
    }
}

impl_differ_simple!(VideoPlayer);

#[derive(Clone, Deserialize, Serialize, StaticRecord, Differ)]
#[static_record(singleton = SINGLETON)]
pub struct Config {
    pub id: String,

    #[static_record(findable)]
    pub locale: String,

    pub playlist_path: Option<String>,

    pub media_type: MediaType,
    pub root_path: Option<String>,
    pub video_player: VideoPlayer,
    pub video_player_path: Option<String>,
}

impl Config {
    pub fn can_play(&self) -> bool {
        if self.playlist_path.is_some() {
            return true;
        }
        matches!(
            (
                self.media_type,
                self.video_player,
                self.video_player_path.as_ref(),
            ),
            (MediaType::Server, _, _)
                | (MediaType::Image, _, _)
                | (MediaType::Video, VideoPlayer::Native, _)
                | (MediaType::Video, _, Some(_))
        )
    }

    pub fn find_all_paths(&self) -> Vec<PathBuf> {
        self.media_type.find_all_paths(
            self.root_path
                .as_ref()
                .expect("Root path should be available atm"),
        )
    }
}

static SINGLETON: Singleton<Config> = Lazy::new(RwLock::default);