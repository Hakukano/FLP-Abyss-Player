use std::{
    collections::HashMap,
    fs::{read_dir, File},
    ops::Deref,
    path::Path,
};

use once_cell::sync::OnceCell;

use crate::get_cli;

pub mod ui;

pub struct Locale {
    pub ui: ui::Locale,
}

struct Locales(HashMap<String, Locale>);

impl Locales {
    fn new() -> Self {
        let assets_path = get_cli().assets_path.as_str();
        let mut map = HashMap::new();

        let locale_path = Path::new(assets_path).join("locale");
        for entry in read_dir(locale_path)
            .expect("Cannot read directory")
            .flatten()
        {
            if let Ok(metadata) = entry.metadata() {
                if metadata.is_dir() {
                    let locale = entry
                        .file_name()
                        .to_str()
                        .expect("Invalid path")
                        .to_string();
                    map.insert(
                        locale.clone(),
                        Locale {
                            ui: serde_json::from_reader(
                                File::open(entry.path().join("ui.json"))
                                    .expect("Cannot open locale file"),
                            )
                            .expect("Cannot parse locale file"),
                        },
                    );
                }
            }
        }

        Self(map)
    }

    fn get_one(&self) -> &Locale {
        let cli = get_cli();
        self.get(&cli.locale)
            .unwrap_or_else(|| self.get("en_US").expect("locale en_US is not found"))
    }
}

impl Deref for Locales {
    type Target = HashMap<String, Locale>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub fn get() -> &'static Locale {
    static LOCALES: OnceCell<Locales> = OnceCell::new();
    let locales = LOCALES.get_or_init(Locales::new);
    locales.get_one()
}
