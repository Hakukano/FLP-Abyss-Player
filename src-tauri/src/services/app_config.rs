use anyhow::Result;
use tauri::{App, Manager};

use crate::models::app_config::AppConfig;

mod json;

pub trait AppConfigService: Send + Sync {
    fn all(&self) -> AppConfig;

    fn save(&mut self, app_config: AppConfig) -> Result<AppConfig>;
}

fn system_locale() -> String {
    sys_locale::get_locale().unwrap_or_else(|| "en-US".to_string())
}

pub fn instantiate() -> Box<dyn AppConfigService> {
    Box::<json::AppConfigService>::default()
}

pub fn initialize(instance: &mut Box<dyn AppConfigService>, app: &App) {
    *instance = Box::new(json::AppConfigService::load_or_defaults(
        app.path()
            .app_config_dir()
            .expect("App config dir not found")
            .join("app_config.json"),
    ))
}
