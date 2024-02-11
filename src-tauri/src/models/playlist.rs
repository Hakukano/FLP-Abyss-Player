use anyhow::{anyhow, Result};
use chrono::{DateTime, Utc};
use serde::Serialize;
use std::cmp::Ordering;

#[derive(Clone, Copy)]
enum MetaCmpBy {
    Path,
    CreateAt,
    UpdateAt,
}

impl MetaCmpBy {
    fn parse(s: impl AsRef<str>) -> Result<Self> {
        match s.as_ref() {
            "path" => Ok(MetaCmpBy::Path),
            "created_at" => Ok(MetaCmpBy::CreateAt),
            "updated_at" => Ok(MetaCmpBy::UpdateAt),
            _ => Err(anyhow!("Invalid MetaCmpBy")),
        }
    }
}

#[derive(Serialize)]
struct Meta {
    path: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl Meta {
    fn cmp_by(&self, other: &Meta, by: MetaCmpBy, ascend: bool) -> Ordering {
        match by {
            MetaCmpBy::Path => {
                if ascend {
                    self.path.cmp(&other.path)
                } else {
                    other.path.cmp(&self.path)
                }
            }
            MetaCmpBy::CreateAt => {
                if ascend {
                    self.created_at.cmp(&other.created_at)
                } else {
                    other.created_at.cmp(&self.created_at)
                }
            }
            MetaCmpBy::UpdateAt => {
                if ascend {
                    self.updated_at.cmp(&other.updated_at)
                } else {
                    other.updated_at.cmp(&self.updated_at)
                }
            }
        }
    }
}

#[derive(Serialize)]
struct Entry {
    meta: Meta,
}

impl Entry {
    fn new(meta: Meta) -> Self {
        Self { meta }
    }

    fn matches_group(&self, group: impl AsRef<str>) -> bool {
        self.meta.path.starts_with(group.as_ref())
    }
}

#[derive(Serialize)]
struct EntryGroup {
    meta: Meta,
    entries: Vec<Entry>,
}

impl EntryGroup {
    fn new(meta: Meta) -> Self {
        Self {
            meta,
            entries: Vec::new(),
        }
    }

    fn take_matched_entries(&mut self, entries: Vec<Entry>) -> Vec<Entry> {
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

    fn sort_by(&mut self, by: MetaCmpBy, ascend: bool) {
        self.entries
            .sort_by(|a, b| a.meta.cmp_by(&b.meta, by, ascend))
    }
}

#[derive(Default)]
struct EntryGroups(Vec<EntryGroup>);

impl EntryGroups {
    fn create_groups(&mut self, metas: Vec<Meta>) {
        self.0
            .append(&mut metas.into_iter().map(EntryGroup::new).collect());
    }

    fn create_entries(&mut self, metas: Vec<Meta>) {
        let mut entries = metas.into_iter().map(Entry::new).collect::<Vec<_>>();
        for entry_group in self.0.iter_mut() {
            entries = entry_group.take_matched_entries(entries);
        }
    }

    fn sort_by(&mut self, by: MetaCmpBy, ascend: bool) {
        self.0.sort_by(|a, b| a.meta.cmp_by(&b.meta, by, ascend))
    }
}

pub trait Playlist {}
