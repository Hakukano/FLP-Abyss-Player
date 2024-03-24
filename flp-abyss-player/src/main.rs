#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

#[macro_use]
extern crate rust_i18n;

#[macro_use]
extern crate flp_abyss_player_derive;

mod library;
mod models;
mod views;

const VERSION_MAJOR: &str = env!("CARGO_PKG_VERSION_MAJOR");
const VERSION_MINOR: &str = env!("CARGO_PKG_VERSION_MINOR");
const VERSION_PATCH: &str = env!("CARGO_PKG_VERSION_PATCH");

i18n!(fallback = "en_US");

fn main() {
    // Init locale
    rust_i18n::set_locale(models::config::Config::locale().as_str());

    views::Task::run().join();
}
