#[macro_use]
extern crate rust_i18n;

use tauri::{Manager, State};

mod models;
mod state;

i18n!(fallback = "en_US");

#[tauri::command]
fn greet(name: &str, app_state: State<state::AppState>) -> String {
    format!(
        "{}: {}, {}",
        t!("ui.app_name"),
        name,
        app_state.app_config.read().locale()
    )
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .manage(state::AppState::new())
        .setup(|app| {
            models::app_config::initialize(
                &mut app.state::<state::AppState>().app_config.write(),
                app,
            );
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
