#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

#[macro_use]
extern crate rust_i18n;

mod controller;
mod library;
mod model;
mod view;
mod widget;

use clap::Parser;
use once_cell::sync::Lazy;
use std::sync::mpsc::channel;

const VERSION_MAJOR: &str = env!("CARGO_PKG_VERSION_MAJOR");
const VERSION_MINOR: &str = env!("CARGO_PKG_VERSION_MINOR");
const VERSION_PATCH: &str = env!("CARGO_PKG_VERSION_PATCH");

i18n!(fallback = "en_US");

fn value_parser_auto_interval(s: &str) -> Result<u32, String> {
    let auto_interval = s.parse::<u32>().map_err(|err| err.to_string())?;
    if model::config::AUTO_INTERVAL_RANGE.contains(&auto_interval) {
        Ok(auto_interval)
    } else {
        Err(format!(
            "auto_interval should be in the range of {:?} but found {}",
            model::config::AUTO_INTERVAL_RANGE,
            auto_interval
        ))
    }
}

#[derive(Clone, Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// [Startup] The path to assets directory
    #[arg(long)]
    pub assets_path: String,

    /// [Startup] The locale to use. e.g. en_US. Default to system locale
    #[arg(long)]
    pub locale: Option<String>,

    /// [Startup] Path to json config file
    #[arg(long)]
    pub config_file: Option<String>,

    /// [Player] Repeat on one media
    #[arg(long, default_value_t = false)]
    pub repeat: bool,

    /// [Player] Auto play
    #[arg(long, default_value_t = false)]
    pub auto: bool,

    /// [Player] The interva of seconds for auto play to next media. Range: (1..=60)
    #[arg(long, default_value_t = 1, value_parser = value_parser_auto_interval)]
    pub auto_interval: u32,

    /// [Player] Loop the playlist
    #[arg(long = "loop", default_value_t = false)]
    pub lop: bool,

    /// [Player] Randomize next media
    #[arg(long, default_value_t = false)]
    pub random: bool,

    /// [Naviation] Path to playlist file. This option will overwrite all other navigation options
    #[arg(long)]
    pub playlist_path: Option<String>,

    /// [Naviation] The media type to be played
    #[arg(long, value_enum, default_value_t = model::config::MediaType::Unset)]
    pub media_type: model::config::MediaType,

    /// [Naviation] The root path
    #[arg(long)]
    pub root_path: Option<String>,

    /// [Naviation] The video player to use for playing videos
    #[arg(long, value_enum, default_value_t = model::config::VideoPlayer::Unset)]
    pub video_player: model::config::VideoPlayer,

    /// [Naviation] The video player executable path
    #[arg(long)]
    pub video_player_path: Option<String>,
}

static CLI: Lazy<Cli> = Lazy::new(Cli::parse);

fn main() {
    // Init locale
    rust_i18n::set_locale(model::config::Config::locale().as_str());

    let (command_tx, command_rx) = channel::<controller::Command>();
    let (packet_tx, packet_rx) = channel::<view::Packet>();

    let controller_task = controller::Task::run(command_rx, packet_tx.clone());
    let view_task = view::Task::run(packet_rx, packet_tx, command_tx);

    let _ = view_task.join();
    let _ = controller_task.join();
}
