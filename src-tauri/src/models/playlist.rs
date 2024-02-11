use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(Serialize)]
struct Entry {
    path: String,
    updated_at: DateTime<Utc>,
}

impl Entry {
    fn new(path: String, updated_at: DateTime<Utc>) -> Self {
        Self { path, updated_at }
    }

    fn matches_group(&self, group: impl AsRef<str>) -> bool {
        self.path.starts_with(group.as_ref())
    }
}

#[derive(Serialize)]
struct EntryGroup {
    path: String,
    updated_at: DateTime<Utc>,
    entries: Vec<Entry>,
}

impl EntryGroup {
    fn new(path: String, updated_at: DateTime<Utc>) -> Self {
        Self {
            path,
            updated_at,
            entries: Vec::new(),
        }
    }

    fn take_matched_entries(&mut self, entries: Vec<Entry>) -> Vec<Entry> {
        let mut remainders = Vec::new();
        for entry in entries.into_iter() {
            if entry.matches_group(self.path.as_str()) {
                self.entries.push(entry);
            } else {
                remainders.push(entry);
            }
        }
        remainders
    }

    fn sort_by_name(&mut self, ascend: bool) {
        self.entries.sort_by(|a, b| {
            if ascend {
                a.path.cmp(&b.path)
            } else {
                b.path.cmp(&a.path)
            }
        })
    }

    fn sort_by_time(&mut self, ascend: bool) {
        self.entries.sort_by(|a, b| {
            if ascend {
                a.updated_at.cmp(&b.updated_at)
            } else {
                b.updated_at.cmp(&a.updated_at)
            }
        })
    }
}

#[derive(Default)]
struct EntryGroups(Vec<EntryGroup>);

impl EntryGroups {
    fn create_groups(&mut self, groups: Vec<(String, DateTime<Utc>)>) {
        self.0.append(
            &mut groups
                .into_iter()
                .map(|group| EntryGroup::new(group.0, group.1))
                .collect(),
        );
    }

    fn create_entries(&mut self, entries: Vec<(String, DateTime<Utc>)>) {
        let mut entries = entries
            .into_iter()
            .map(|entry| Entry::new(entry.0, entry.1))
            .collect::<Vec<_>>();
        for entry_group in self.0.iter_mut() {
            entries = entry_group.take_matched_entries(entries);
        }
    }

    fn sort_by_name(&mut self, ascend: bool) {
        self.0.sort_by(|a, b| {
            if ascend {
                a.path.cmp(&b.path)
            } else {
                b.path.cmp(&a.path)
            }
        })
    }

    fn sort_by_time(&mut self, ascend: bool) {
        self.0.sort_by(|a, b| {
            if ascend {
                a.updated_at.cmp(&b.updated_at)
            } else {
                b.updated_at.cmp(&a.updated_at)
            }
        })
    }
}

pub trait Playlist {}
