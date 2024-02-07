#[macro_use]
extern crate rust_i18n;

#[macro_use]
extern crate tracing;

use tauri::Manager;

mod controllers;
mod models;
mod utils;

i18n!(fallback = "en_US");

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tracing_subscriber::fmt::init();

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .manage(models::Models::new())
        .setup(|app| {
            models::app_config::initialize(
                &mut app.state::<models::Models>().app_config.write(),
                app,
            );
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![controllers::api])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
