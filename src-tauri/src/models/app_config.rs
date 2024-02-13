use anyhow::Result;
use serde_json::Value;
use std::path::PathBuf;
use tauri::{App, Manager};

mod json;
#[cfg(test)]
mod tests;

pub trait AppConfig: Send + Sync {
    fn locale(&self) -> String;
    fn set_locale(&mut self, locale: String);

    fn playlist(&self) -> Option<PathBuf>;
    fn set_playlist(&mut self, playlist: Option<PathBuf>) -> Result<()>;

    fn to_json(&self) -> Result<Value>;
    fn set_from_json(&mut self, value: Value) -> Result<()>;
}

fn system_locale() -> String {
    sys_locale::get_locale().unwrap_or_else(|| "en-US".to_string())
}

pub fn instantiate() -> Box<dyn AppConfig> {
    if cfg!(test) {
        Box::new(json::AppConfig::load_or_defaults(""))
    } else {
        Box::<json::AppConfig>::default()
    }
}

pub fn initialize(instance: &mut Box<dyn AppConfig>, app: &App) {
    *instance = Box::new(json::AppConfig::load_or_defaults(
        app.path()
            .app_config_dir()
            .expect("App config dir not found")
            .join("app_config.json"),
    ))
}
