use std::{collections::HashMap, env, fs::File, ops::Deref, path::Path};

use once_cell::sync::OnceCell;

pub mod ui;

macro_rules! read_locale {
    ($map:ident, $locale:expr, $(($name:ident, $sub_path:expr),)+) => {
        let locale_path_env =
            env::var("LOCALE_PATH")
            .unwrap_or("./assets/locale".to_string());
        let locale_path = Path::new(locale_path_env.as_str());
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
    ($map:ident, $($locale:expr,)+) => {
        $(
            read_locale!($map, $locale, (ui, "ui"),);
        )+
    }
}

pub struct Locale {
    pub ui: ui::Locale,
}

struct Locales(HashMap<String, Locale>);

impl Locales {
    fn new() -> Self {
        let mut map = HashMap::new();
        read_locales!(map, "en_us",);
        Self(map)
    }

    fn get_one(&self) -> &Locale {
        let locale_env = env::var("LOCALE").unwrap_or("en_us".to_string());
        self.get(locale_env.as_str()).expect("Unknown LOCALE")
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
