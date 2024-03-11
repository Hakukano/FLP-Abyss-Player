#[macro_use]
extern crate tracing;

use tauri::Manager;
use tracing::Level;

mod controllers;
mod models;
mod services;
mod utils;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let _guard = utils::init_tracing(
        cfg!(debug_assertions),
        if cfg!(debug_assertions) {
            Level::DEBUG
        } else {
            Level::INFO
        },
    );

    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_shell::init())
        .manage(services::Services::new())
        .setup(|app| {
            models::app_config::initialize(
                &mut app.state::<services::Services>().app_config.write(),
                app,
            );
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![controllers::api::api])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
