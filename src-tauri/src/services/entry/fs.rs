use anyhow::Result;
use walkdir::WalkDir;

use crate::{
    models::entry::Entry,
    utils::{
        match_mime,
        meta::{Meta, MetaCmpBy},
    },
};

#[derive(Default)]
pub struct EntryService {
    data: Vec<Entry>,
}

impl super::EntryService for EntryService {
    fn all(&self) -> Vec<Entry> {
        self.data.clone()
    }

    fn save(&mut self, entry: Entry) -> Result<Entry> {
        if let Some(origin) = self.data.iter_mut().find(|g| g.id == entry.id) {
            *origin = entry.clone();
        } else {
            self.data.push(entry.clone());
        }
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
                                Some(Entry::new(meta.clone(), mime, None))
                            } else {
                                None
                            }
                        })
                })
            })
            .collect()
    }

    fn sort(&mut self, by: MetaCmpBy, ascend: bool) {
        self.data.sort_by(|a, b| a.meta.cmp_by(&b.meta, by, ascend));
    }
}
