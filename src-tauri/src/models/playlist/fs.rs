use crate::shared::system_time_to_utc;
use anyhow::Result;
use std::collections::HashSet;
use walkdir::WalkDir;

use super::{entry::Entry, group::Group, Meta};

#[derive(Default)]
pub struct Playlist {
    groups: Vec<Group>,
}

impl super::Playlist for Playlist {
    fn scan(&self, root_path: String, allowed_mimes: Vec<String>) -> Vec<Entry> {
        let allowed_mimes_set = allowed_mimes.into_iter().collect::<HashSet<_>>();
        WalkDir::new(root_path)
            .into_iter()
            .filter_map(|err| err.ok())
            .filter_map(|entry| {
                if let (Ok(meta), Some(path)) = (
                    entry.metadata(),
                    entry.path().to_str().map(|p| p.to_string()),
                ) {
                    mime_guess::from_path(entry.path())
                        .into_iter()
                        .find_map(|guess| {
                            let mime = guess.to_string();
                            if allowed_mimes_set.contains(&mime) {
                                Some(Entry::new(
                                    Meta {
                                        path,
                                        created_at: meta
                                            .created()
                                            .map(|time| {
                                                system_time_to_utc(&time).unwrap_or_default()
                                            })
                                            .unwrap_or_default(),
                                        updated_at: meta
                                            .modified()
                                            .map(|time| {
                                                system_time_to_utc(&time).unwrap_or_default()
                                            })
                                            .unwrap_or_default(),
                                    },
                                    mime,
                                ))
                            } else {
                                None
                            }
                        })
                } else {
                    None
                }
            })
            .collect()
    }

    fn create_groups(&mut self, paths: Vec<String>) -> Result<()> {
        todo!()
    }

    fn groups(&self) -> &Vec<Group> {
        &self.groups
    }

    fn groups_mut(&mut self) -> &mut Vec<Group> {
        &mut self.groups
    }
}
