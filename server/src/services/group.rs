use anyhow::anyhow;
use once_cell::sync::Lazy;
use parking_lot::RwLock;
use serde_json::Value;

use crate::{models::group::Group, utils::meta::MetaCmpBy};

pub type SaveError = ();

pub type DestroyError = ();

pub fn all() -> Vec<Group> {
    INSTANCE.read().clone()
}

pub fn find(id: &str) -> Option<Group> {
    INSTANCE.read().iter().find(|group| group.id == id).cloned()
}

pub fn save(group: &Group) -> Result<(), SaveError> {
    for existing in INSTANCE.write().iter_mut() {
        if existing.id == group.id {
            *existing = group.clone();
            return Ok(());
        }
    }
    INSTANCE.write().push(group.clone());
    Ok(())
}

pub fn destroy(id: &str) -> Result<(), DestroyError> {
    INSTANCE.write().retain(|group| group.id != id);
    Ok(())
}

pub fn to_json() -> Value {
    serde_json::to_value(INSTANCE.read().clone()).expect("Corrupted group data")
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
    let mut groups = Group::all();
    groups
        .iter()
        .position(|group| group.id == id)
        .map(|index| {
            let new_index = (index as i64 + offset).max(0).min(groups.len() as i64 - 1) as usize;
            let deleted = groups.remove(index);
            groups.insert(new_index, deleted);
            *INSTANCE.write() = groups;
        })
        .ok_or_else(|| anyhow!("Not found"))
}

static INSTANCE: Lazy<RwLock<Vec<Group>>> = Lazy::new(RwLock::default);
