use anyhow::Result;
use std::path::PathBuf;
use tauri::{App, Manager};

mod json;

pub trait AppConfig: Send + Sync {
    fn locale(&self) -> String;
    fn set_locale(&mut self, locale: String);

    fn root_path(&self) -> Option<PathBuf>;
    fn set_root_path(&mut self, root_path: Option<PathBuf>) -> Result<()>;
}

fn system_locale() -> String {
    sys_locale::get_locale()
        .unwrap_or_else(|| "en_US".to_string())
        .replace('-', "_")
}

fn file_path(app: &App) -> PathBuf {
    if cfg!(test) {
        app.path()
            .temp_dir()
            .expect("Temp dir not found")
            .join("app_config.json")
    } else {
        app.path()
            .app_config_dir()
            .expect("App config dir not found")
            .join("app_config.json")
    }
}

pub fn instantiate() -> Box<dyn AppConfig> {
    Box::<json::SystemConfig>::default()
}

pub fn initialize(instance: &mut Box<dyn AppConfig>, app: &App) {
    *instance = Box::new(json::SystemConfig::load_or_defaults(file_path(app)))
}
