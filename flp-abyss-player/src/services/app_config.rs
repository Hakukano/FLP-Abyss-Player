use anyhow::Result;

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
