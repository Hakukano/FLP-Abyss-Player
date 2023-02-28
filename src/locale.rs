use std::{collections::HashMap, fs::File, ops::Deref, path::Path};

use once_cell::sync::OnceCell;

use crate::get_cli;

pub mod ui;

macro_rules! read_locale {
    ($cli:ident, $map:ident, $locale:expr, $(($name:ident, $sub_path:expr),)+) => {
        let locale_path = Path::new($cli.assets_path.as_str()).join("locale");
        $map.insert(
            $locale.to_string(),
            Locale {
                $(
                    $name: serde_json::from_reader(File::open(locale_path
                            .join($sub_path)
                            .join(format!("{}.json", $locale)))
                        .expect("Cannot open locale file"))
                    .expect("Cannot parse locale file"),
                )+
            }
        );
    }
}

macro_rules! read_locales {
    ($cli:ident, $map:ident, $($locale:expr,)+) => {
        $(
            read_locale!($cli, $map, $locale, (ui, "ui"),);
        )+
    }
}

pub struct Locale {
    pub ui: ui::Locale,
}

struct Locales(HashMap<String, Locale>);

impl Locales {
    fn new() -> Self {
        let cli = get_cli();
        let mut map = HashMap::new();
        read_locales!(cli, map, "en_us", "ja_jp",);
        Self(map)
    }

    fn get_one(&self) -> &Locale {
        let cli = get_cli();
        self.get(&cli.locale).expect("Unknown LOCALE")
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
