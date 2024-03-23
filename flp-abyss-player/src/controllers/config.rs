use serde_json::Value;

use crate::{
    library::differ::Differ,
    models::config::Config,
    views::{Method, Packet},
};

pub fn index() -> Packet {
    Packet::new(Method::Got, Config::all())
}

pub fn get(id: &str) -> Packet {
    Config::find(id)
        .map(|config| Packet::new(Method::Got, config))
        .unwrap_or_else(|| Packet::new(Method::Error, t!("config.error.not_found").as_ref()))
}

pub fn update(id: &str, diff: Value) -> Packet {
    if let Some(mut config) = Config::find(id) {
        config.apply_diff(diff);
        config.save();
        Packet::new(Method::Updated, ())
    } else {
        Packet::new(Method::Error, t!("config.error.not_found").as_ref())
    }
}
