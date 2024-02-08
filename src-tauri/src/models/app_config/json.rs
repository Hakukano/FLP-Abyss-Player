use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{
    fs::File,
    io::{BufReader, Read},
    path::{Path, PathBuf},
};

#[derive(Deserialize, Serialize)]
pub struct AppConfig {
    locale: String,
    root_path: Option<String>,
}

impl AppConfig {
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

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            locale: super::system_locale(),
            root_path: None,
        }
    }
}

impl super::AppConfig for AppConfig {
    fn locale(&self) -> String {
        self.locale.clone()
    }

    fn set_locale(&mut self, locale: String) {
        self.locale = locale;
        rust_i18n::set_locale(self.locale().as_str());
    }

    fn root_path(&self) -> Option<PathBuf> {
        self.root_path
            .as_ref()
            .map(|p| Path::new(p.as_str()).into())
    }

    fn set_root_path(&mut self, root_path: Option<PathBuf>) -> Result<()> {
        self.root_path = root_path
            .as_ref()
            .map(|p| {
                p.to_str()
                    .ok_or_else(|| anyhow!("Invalid root path: {:?}", root_path))
                    .map(|s| s.to_string())
            })
            .transpose()?;
        Ok(())
    }

    fn to_json(&self) -> Result<Value> {
        Ok(serde_json::to_value(self)?)
    }

    fn set_from_json(&mut self, value: Value) -> Result<()> {
        *self = serde_json::from_value(value)?;
        Ok(())
    }
}
