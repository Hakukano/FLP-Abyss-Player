use anyhow::Result;
use std::collections::HashMap;
use walkdir::WalkDir;

use crate::{
    models::entry::Entry,
    utils::{match_mime, meta::Meta},
};

pub struct EntryService {
    data: HashMap<String, Entry>,
}

impl super::EntryService for EntryService {
    fn all(&self) -> Vec<Entry> {
        self.data.values().cloned().collect()
    }

    fn save(&mut self, entry: Entry) -> Result<Entry> {
        self.data.insert(entry.id.clone(), entry.clone());
        Ok(entry)
    }

    fn scan(&self, root_path: String, allowed_mimes: Vec<String>) -> Vec<Entry> {
        WalkDir::new(root_path)
            .into_iter()
            .filter_map(|err| err.ok())
            .filter_map(|entry| {
                Meta::from_path(entry.path()).ok().and_then(|meta| {
                    mime_guess::from_path(entry.path())
                        .into_iter()
                        .find_map(|guess| {
                            let mime = guess.to_string();
                            if match_mime(mime.as_str(), allowed_mimes.as_slice()) {
                                Some(Entry::new(meta, mime, None))
                            } else {
                                None
                            }
                        })
                })
            })
            .collect()
    }
}
