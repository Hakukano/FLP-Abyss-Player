use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{models::entry::Entry, utils::meta::MetaCmpBy};

#[derive(Deserialize, Serialize)]
pub struct EntryService {
    data: Vec<Entry>,
    last_sort_by: MetaCmpBy,
    last_sort_ascend: bool,
}

impl Default for EntryService {
    fn default() -> Self {
        Self {
            data: Vec::new(),
            last_sort_by: MetaCmpBy::Path,
            last_sort_ascend: true,
        }
    }
}

impl super::EntryService for EntryService {
    fn all(&self) -> Vec<Entry> {
        self.data.clone()
    }

    fn set_all(&mut self, entries: Vec<Entry>) {
        self.data = entries;
        self.last_sort_by = MetaCmpBy::Default;
        self.last_sort_ascend = true;
    }

    fn save(&mut self, entry: Entry) -> Result<Entry> {
        if let Some(origin) = self.data.iter_mut().find(|g| g.id == entry.id) {
            *origin = entry.clone();
        } else {
            self.data.push(entry.clone());
            self.sort(self.last_sort_by, self.last_sort_ascend);
        }
        Ok(entry)
    }

    fn sort(&mut self, by: MetaCmpBy, ascend: bool) {
        self.data.sort_by(|a, b| a.meta.cmp_by(&b.meta, by, ascend));
        self.last_sort_by = by;
        self.last_sort_ascend = ascend;
    }

    fn destroy(&mut self, id: &str) -> Result<Entry> {
        let (index, _) = self
            .data
            .iter()
            .enumerate()
            .find(|(_, e)| e.id == id)
            .ok_or_else(|| anyhow!("Playlist not found"))?;
        Ok(self.data.remove(index))
    }

    fn to_json(&self) -> Value {
        serde_json::to_value(self).expect("Corrupted playlist data")
    }

    fn set_json(&mut self, value: Value) -> Result<()> {
        *self = serde_json::from_value(value)?;
        Ok(())
    }
}
