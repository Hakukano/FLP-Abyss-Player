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
    playlist: Option<String>,
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
            playlist: None,
        }
    }
}

impl super::AppConfig for AppConfig {
    fn locale(&self) -> String {
        self.locale.clone()
    }

    fn set_locale(&mut self, locale: String) {
        self.locale = locale;
    }

    fn playlist(&self) -> Option<PathBuf> {
        self.playlist.as_ref().map(|p| Path::new(p.as_str()).into())
    }

    fn set_playlist(&mut self, playlist: Option<PathBuf>) -> Result<()> {
        self.playlist = playlist
            .as_ref()
            .map(|p| {
                p.to_str()
                    .ok_or_else(|| anyhow!("Invalid path: {:?}", playlist))
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

#[cfg(test)]
mod tests {
    use serde_json::json;
    use std::path::Path;
    use tap::Tap;

    use super::*;
    use crate::models::app_config::system_locale;

    fn app_config_default() -> Box<dyn super::super::AppConfig> {
        Box::<AppConfig>::default()
    }

    #[test]
    fn locale() {
        let lhs = app_config_default().locale();
        let rhs = system_locale();
        assert_eq!(lhs, rhs);
    }

    #[test]
    fn set_locale() {
        let lhs = app_config_default()
            .tap_mut(|config| config.set_locale("xx_YY".to_string()))
            .locale();
        let rhs = "xx_YY";
        assert_eq!(lhs, rhs);
    }

    #[test]
    fn playlist() {
        let lhs = app_config_default().playlist();
        let rhs = None;
        assert_eq!(lhs, rhs);
    }

    #[test]
    fn set_playlist() {
        let lhs = app_config_default()
            .tap_mut(|config| config.set_playlist(Some("/test".into())).unwrap())
            .playlist();
        let rhs = Some("/test".into());
        assert_eq!(lhs, rhs)
    }

    #[test]
    fn to_json() {
        let lhs = app_config_default().to_json().unwrap();
        let rhs = json!({
            "locale": system_locale(),
            "playlist": null,
        });
        assert_eq!(lhs, rhs);
    }

    #[test]
    fn set_from_json() {
        let lhs = app_config_default().tap_mut(|config| {
            config
                .set_from_json(json!({
                    "locale": "xx_YY",
                    "playlist": "/test",
                }))
                .unwrap();
        });
        let lhs = (lhs.locale(), lhs.playlist());
        let rhs = ("xx_YY".to_string(), Some(Path::new("/test").to_path_buf()));
        assert_eq!(lhs, rhs);
    }
}
