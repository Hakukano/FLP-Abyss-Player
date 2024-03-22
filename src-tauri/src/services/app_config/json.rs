use anyhow::Result;
use serde::{Deserialize, Serialize};

use std::{
    fs::File,
    io::{BufReader, Read},
    path::Path,
};

use crate::models::app_config::AppConfig;

#[derive(Deserialize, Serialize)]
pub struct AppConfigService {
    data: AppConfig,
}

impl AppConfigService {
    fn new(s: impl AsRef<str>) -> Self {
        serde_json::from_str(s.as_ref()).expect("Cannot parse json for SystemConfig")
    }

    fn load(path: impl AsRef<Path>) -> Result<String> {
        let mut data = String::new();
        BufReader::new(File::open(path)?).read_to_string(&mut data)?;
        Ok(data)
    }

    pub fn load_or_defaults(path: impl AsRef<Path>) -> Self {
        if let Ok(data) = Self::load(path) {
            Self::new(data)
        } else {
            Self::default()
        }
    }
}

impl Default for AppConfigService {
    fn default() -> Self {
        Self {
            data: AppConfig {
                locale: super::system_locale(),
            },
        }
    }
}

impl super::AppConfigService for AppConfigService {
    fn all(&self) -> AppConfig {
        self.data.clone()
    }

    fn save(&mut self, app_config: AppConfig) -> Result<AppConfig> {
        self.data = app_config.clone();
        Ok(app_config)
    }
}
