use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::{
    fs::File,
    io::{BufReader, Read},
    path::{Path, PathBuf},
};

#[derive(Default, Deserialize, Serialize)]
pub struct SystemConfig {
    locale: String,
    root_path: Option<String>,
}

impl SystemConfig {
    fn new(s: impl AsRef<str>) -> Self {
        serde_json::from_str(s.as_ref()).expect("Cannot parse json for SystemConfig")
    }

    fn load(path: impl AsRef<Path>) -> Result<String> {
        let mut data = String::new();
        BufReader::new(File::open(path)?).read_to_string(&mut data)?;
        Ok(data)
    }

    pub fn load_or_defaults(path: impl AsRef<Path>) -> Self {
        Self::new(Self::load(path).unwrap_or_else(|_| {
            json!({
                "locale": super::system_locale()
            })
            .to_string()
        }))
    }
}

impl super::AppConfig for SystemConfig {
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
}
