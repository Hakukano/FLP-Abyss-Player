use anyhow::Result;
use once_cell::sync::Lazy;
use parking_lot::RwLock;

use crate::models::app_config::AppConfig;

pub fn all() -> AppConfig {
    INSTANCE.read().clone()
}

pub fn save(app_config: &AppConfig) -> Result<()> {
    *INSTANCE.write() = app_config.clone();
    Ok(())
}

fn system_locale() -> String {
    sys_locale::get_locale().unwrap_or_else(|| "en-US".to_string())
}

static INSTANCE: Lazy<RwLock<AppConfig>> = Lazy::new(|| {
    RwLock::new(AppConfig {
        locale: system_locale(),
    })
});
