use anyhow::anyhow;
use once_cell::sync::Lazy;
use parking_lot::RwLock;
use serde_json::Value;

use crate::{models::entry::Entry, utils::meta::MetaCmpBy};

pub type SaveError = ();

pub type DestroyError = ();

pub fn all() -> Vec<Entry> {
    INSTANCE.read().clone()
}

pub fn find(id: &str) -> Option<Entry> {
    INSTANCE.read().iter().find(|entry| entry.id == id).cloned()
}

pub fn save(entry: Entry) -> Result<Entry, SaveError> {
    for existing in INSTANCE.write().iter_mut() {
        if existing.id == entry.id {
            *existing = entry.clone();
            return Ok(entry);
        }
    }
    INSTANCE.write().push(entry.clone());
    Ok(entry)
}

pub fn destroy(id: &str) -> Result<(), DestroyError> {
    INSTANCE.write().retain(|entry| entry.id != id);
    Ok(())
}

pub fn to_json() -> Value {
    serde_json::to_value(INSTANCE.read().clone()).expect("Corrupted entry data")
}

pub fn set_json(value: Value) -> anyhow::Result<()> {
    *INSTANCE.write() = serde_json::from_value(value)?;
    Ok(())
}

pub fn sort(by: MetaCmpBy, ascend: bool) {
    INSTANCE
        .write()
        .sort_by(|a, b| a.meta.cmp_by(&b.meta, by, ascend));
}

pub fn shift(id: &str, offset: i64) -> anyhow::Result<()> {
    let mut entries = Entry::all();
    entries
        .iter()
        .position(|entry| entry.id == id)
        .map(|index| {
            let new_index = (index as i64 + offset).max(0).min(entries.len() as i64 - 1) as usize;
            let deleted = entries.remove(index);
            entries.insert(new_index, deleted);
            *INSTANCE.write() = entries;
        })
        .ok_or_else(|| anyhow!("Not found"))
}

static INSTANCE: Lazy<RwLock<Vec<Entry>>> = Lazy::new(RwLock::default);
