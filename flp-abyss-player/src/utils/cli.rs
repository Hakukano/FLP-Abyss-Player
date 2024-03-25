use clap::{command, Parser};
use once_cell::sync::Lazy;

use crate::models::config::{MediaType, VideoPlayer};

#[derive(Clone, Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// [Startup] The path to assets directory
    #[arg(long, default_value_t = String::from("./assets"))]
    pub assets_path: String,

    /// [Startup] The locale to use. e.g. en_US. Default to system locale
    #[arg(long)]
    pub locale: Option<String>,

    /// [Startup] Path to json config file
    #[arg(long)]
    pub config_file: Option<String>,

    /// [Navigation] Path to playlist file. This option will overwrite all other navigation options
    #[arg(long)]
    pub playlist_path: Option<String>,

    /// [Navigation] The media type to be played
    #[arg(long, value_enum, default_value_t = MediaType::Unset)]
    pub media_type: MediaType,

    /// [Navigation] The root path
    #[arg(long)]
    pub root_path: Option<String>,

    /// [Navigation] The video player to use for playing videos
    #[arg(long, value_enum, default_value_t = VideoPlayer::Unset)]
    pub video_player: VideoPlayer,

    /// [Navigation] The video player executable path
    #[arg(long)]
    pub video_player_path: Option<String>,
}

pub static CLI: Lazy<Cli> = Lazy::new(Cli::parse);
