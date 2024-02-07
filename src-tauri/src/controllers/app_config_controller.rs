use crate::models;
use tauri::State;

#[tauri::command]
pub fn greet(name: &str, models: State<models::Models>) -> String {
    format!(
        "{}: {}, {}",
        t!("ui.app_name"),
        name,
        models.app_config.read().locale()
    )
}
