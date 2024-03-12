use anyhow::Result;

use crate::{models::entry::Entry, utils::meta::MetaCmpBy};

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

    fn sort(&mut self, by: MetaCmpBy, ascend: bool);

    fn destroy(&mut self, id: &str) -> Result<Entry>;
}

pub fn instantiate() -> Box<dyn EntryService> {
    Box::<fs::EntryService>::default()
}
