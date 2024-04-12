use std::collections::HashMap;

use once_cell::sync::Lazy;
use parking_lot::RwLock;
use serde_json::Value;

use crate::models::playlist::Playlist;

pub type SaveError = ();

pub type DestroyError = ();

pub fn all() -> Vec<Playlist> {
    INSTANCE.read().values().cloned().collect()
}

pub fn find(id: &str) -> Option<Playlist> {
    INSTANCE.read().get(id).cloned()
}

pub fn save(playlist: &Playlist) -> Result<(), SaveError> {
    INSTANCE
        .write()
        .insert(playlist.id.clone(), playlist.clone());
    Ok(())
}

pub fn destroy(id: &str) -> Result<(), DestroyError> {
    INSTANCE.write().remove(id);
    Ok(())
}

pub fn to_json() -> Value {
    serde_json::to_value(INSTANCE.read().clone()).expect("Corrupted playlist data")
}

pub fn set_json(value: Value) -> anyhow::Result<()> {
    *INSTANCE.write() = serde_json::from_value(value)?;
    Ok(())
}

static INSTANCE: Lazy<RwLock<HashMap<String, Playlist>>> = Lazy::new(RwLock::default);
