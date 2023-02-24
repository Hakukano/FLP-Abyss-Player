#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

mod app;
mod config;
mod font;
mod locale;

use clap::Parser;
use eframe::egui;
use once_cell::sync::OnceCell;

#[derive(Clone, Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// The path to font directory
    #[arg(long)]
    pub font_path: String,

    /// The font files to be installed, separated by ; and the first one will be used as default
    #[arg(long)]
    pub fonts: String,

    /// The path to locale directory
    #[arg(long)]
    pub locale_path: String,

    /// The locale to use, in lower case. e.g. en_us
    #[arg(long)]
    pub locale: String,

    /// Path to json config file
    #[arg(long)]
    pub config_file: Option<String>,

    /// The media type to be played
    #[arg(long, value_enum)]
    pub media_type: Option<config::MediaType>,

    /// The root path
    #[arg(long)]
    pub root_path: Option<String>,

    /// The video player to use for playing videos
    #[arg(long, value_enum)]
    pub video_player: Option<config::VideoPlayer>,
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
