#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

mod app;
mod config;
mod font;
mod helper;
mod locale;
mod widget;

use clap::Parser;
use eframe::egui;
use once_cell::sync::OnceCell;

fn value_parser_auto_interval(s: &str) -> Result<u32, String> {
    let auto_interval = s.parse::<u32>().map_err(|err| err.to_string())?;
    if config::AUTO_INTERVAL_RANGE.contains(&auto_interval) {
        Ok(auto_interval)
    } else {
        Err(format!(
            "auto_interval should be in the range of {:?} but found {}",
            config::AUTO_INTERVAL_RANGE,
            auto_interval
        ))
    }
}

#[derive(Clone, Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// The path to assets directory
    #[arg(long)]
    pub assets_path: String,

    /// The font files to be installed, separated by ; and the first one will be used as default
    #[arg(long)]
    pub fonts: String,

    /// The locale to use, in lower case. e.g. en_us
    #[arg(long)]
    pub locale: String,

    /// Path to json config file
    #[arg(long)]
    pub config_file: Option<String>,

    /// The media type to be played
    #[arg(long, value_enum, default_value_t = config::MediaType::Unset)]
    pub media_type: config::MediaType,

    /// The root path
    #[arg(long)]
    pub root_path: Option<String>,

    /// Repeat on one media
    #[arg(long, default_value_t = false)]
    pub repeat: bool,

    /// Auto play
    #[arg(long, default_value_t = false)]
    pub auto: bool,

    /// The interva of seconds for auto play to next media. Range: (1..=60)
    #[arg(long, default_value_t = 1, value_parser = value_parser_auto_interval)]
    pub auto_interval: u32,

    /// Loop the playlist
    #[arg(long = "loop", default_value_t = false)]
    pub lop: bool,

    /// Randomize next media
    #[arg(long, default_value_t = false)]
    pub random: bool,

    /// The video player to use for playing videos
    #[arg(long, value_enum, default_value_t = config::VideoPlayer::Unset)]
    pub video_player: config::VideoPlayer,

    /// The video player executable path
    #[arg(long)]
    pub video_player_path: Option<String>,
}

fn get_cli() -> &'static Cli {
    static INSTANCE: OnceCell<Cli> = OnceCell::new();
    INSTANCE.get_or_init(Cli::parse)
}

fn main() -> eframe::Result<()> {
    // Init and validate cli
    let _ = get_cli();

    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(1600.0, 900.0)),
        ..Default::default()
    };
    eframe::run_native(
        locale::get().ui.app_name.as_str(),
        options,
        Box::new(|cc| Box::new(app::App::new(cc))),
    )
}
