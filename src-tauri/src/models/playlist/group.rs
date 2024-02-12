use serde::Serialize;

use super::{entry::Entry, Meta};

#[derive(Clone, Serialize)]
pub struct Group {
    pub meta: Meta,
    pub entries: Vec<Entry>,
}

impl Group {
    pub fn new(meta: Meta) -> Self {
        Self {
            meta,
            entries: Vec::new(),
        }
    }

    pub fn take_matched_entries(&mut self, entries: Vec<Entry>) -> Vec<Entry> {
        let mut remainders = Vec::new();
        for entry in entries.into_iter() {
            if entry.matches_group(self.meta.path.as_str()) {
                self.entries.push(entry);
            } else {
                remainders.push(entry);
            }
        }
        remainders
    }
}
