use anyhow::Result;

use crate::models::entry::Entry;

mod fs;

pub trait EntryService: Send + Sync {
    fn all(&self) -> Vec<Entry>;

    fn find_by_group_id(&self, group_id: &str) -> Vec<Entry> {
        self.all()
            .iter()
            .filter(|group| group.group_id == group_id)
            .cloned()
            .collect()
    }

    fn save(&mut self, entry: Entry) -> Result<Entry>;

    fn scan(&self, root_path: String, allowed_mimes: Vec<String>) -> Vec<Entry>;
}

pub fn instantiate() -> Box<dyn EntryService> {
    Box::<fs::EntryService>::default()
}
