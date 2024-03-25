pub mod args;
pub mod json;

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
            Self::Unset => "--".to_string(),
            Self::Server => t!("ui.config.media_type.server").to_string(),
            Self::Image => t!("ui.config.media_type.image").to_string(),
            Self::Video => t!("ui.config.media_type.video").to_string(),
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
            Self::Unset => "--".to_string(),
            Self::Native => t!("ui.config.video_player.native").to_string(),
            Self::Vlc => t!("ui.config.video_player.vlc").to_string(),
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

#[cfg(test)]
mod tests {
    use tap::prelude::*;

    use crate::utils::helper::fixtures_dir;

    use super::*;

    pub fn mock_default() -> Config {
        Config {
            id: "default".to_string(),

            locale: "en_US".to_string(),

            playlist_path: None,

            media_type: MediaType::Unset,
            root_path: fixtures_dir().to_str().map(|s| s.to_string()),
            video_player: VideoPlayer::Unset,
            video_player_path: None,
        }
    }

    pub fn mock_image() -> Config {
        mock_default().tap_mut(|config| config.media_type = MediaType::Image)
    }

    #[test]
    fn can_play() {
        assert!(!mock_default().can_play());
        assert!(mock_image().can_play());
    }

    mod media_type {
        use super::*;

        #[test]
        fn default() {
            assert_eq!(MediaType::default(), MediaType::Unset);
        }

        #[test]
        fn is_unset() {
            assert!(MediaType::Unset.is_unset());
            assert!(!MediaType::Image.is_unset());
        }

        #[test]
        fn supported_extensions() {
            assert!(MediaType::Unset.supported_extensions().is_empty());
            assert_eq!(
                MediaType::Image.supported_extensions(),
                &["bmp", "gif", "jpeg", "jpg", "png"]
            );
            assert_eq!(
                MediaType::Server.supported_extensions(),
                &["bmp", "gif", "jpeg", "jpg", "png", "avi", "mp4", "webm", "mp3", "wav",]
            );
            assert_eq!(
                MediaType::Video.supported_extensions(),
                &["avi", "mkv", "mov", "mp4", "webm"]
            );
        }

        #[test]
        fn find_all_paths() {
            let paths = MediaType::Image
                .find_all_paths(fixtures_dir().join("images"))
                .into_iter()
                .map(|p| p.to_str().unwrap().to_string())
                .collect::<Vec<_>>();
            assert_eq!(paths.len(), 9);
        }

        #[test]
        fn convert_u8() {
            assert_eq!(u8::from(MediaType::Unset), 0);
            assert_eq!(MediaType::from(0), MediaType::Unset);
            assert_eq!(u8::from(MediaType::Image), 1);
            assert_eq!(MediaType::from(1), MediaType::Image);
            assert_eq!(u8::from(MediaType::Video), 2);
            assert_eq!(MediaType::from(2), MediaType::Video);
            assert_eq!(u8::from(MediaType::Server), 255);
            assert_eq!(MediaType::from(255), MediaType::Server);
        }
    }

    mod video_player {
        use super::*;

        #[test]
        fn default() {
            assert_eq!(VideoPlayer::default(), VideoPlayer::Unset);
        }

        #[test]
        fn is_unset() {
            assert!(VideoPlayer::Unset.is_unset());
            assert!(!VideoPlayer::Native.is_unset());
        }

        #[test]
        fn convert_u8() {
            assert_eq!(u8::from(VideoPlayer::Unset), 0);
            assert_eq!(VideoPlayer::from(0), VideoPlayer::Unset);
            assert_eq!(u8::from(VideoPlayer::Vlc), 1);
            assert_eq!(VideoPlayer::from(1), VideoPlayer::Vlc);
            assert_eq!(u8::from(VideoPlayer::Native), 255);
            assert_eq!(VideoPlayer::from(255), VideoPlayer::Native);
        }
    }
}
