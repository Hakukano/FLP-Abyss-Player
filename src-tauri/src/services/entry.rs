use anyhow::Result;
use serde_json::Value;

use crate::{models::entry::Entry, utils::meta::MetaCmpBy};

mod memory;

pub trait EntryService: Send + Sync {
    fn all(&self) -> Vec<Entry>;

    fn set_all(&mut self, groups: Vec<Entry>);

    fn find_by_id(&self, id: &str) -> Option<Entry> {
        self.all().iter().find(|entry| entry.id == id).cloned()
    }

    fn find_by_group_id(&self, group_id: &str) -> Vec<Entry> {
        self.all()
            .iter()
            .filter(|group| group.group_id == group_id)
            .cloned()
            .collect()
    }

    fn save(&mut self, entry: Entry) -> Result<Entry>;

    fn sort(&mut self, by: MetaCmpBy, ascend: bool);

    fn destroy(&mut self, id: &str) -> Result<Entry>;

    fn to_json(&self) -> Value;

    fn set_json(&mut self, value: Value) -> Result<()>;
}

pub fn instantiate() -> Box<dyn EntryService> {
    Box::<memory::EntryService>::default()
}
