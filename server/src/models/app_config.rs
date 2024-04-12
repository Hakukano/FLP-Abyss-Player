use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::services::app_config;

#[derive(Clone, Deserialize, Serialize)]
pub struct AppConfig {
    pub locale: String,
}

impl AppConfig {
    pub fn all() -> AppConfig {
        app_config::all()
    }

    pub fn save(&self) -> Result<()> {
        app_config::save(self)
    }
}
