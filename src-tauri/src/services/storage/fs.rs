use std::fs::File;

use anyhow::Result;

use crate::{
    models::storage::Storage,
    services::{entry::EntryService, group::GroupService, playlist::PlaylistService},
};

#[derive(Default)]
pub struct StorageService {}

impl super::StorageService for StorageService {
    fn write(
        &self,
        path: &str,
        playlist_service: &dyn PlaylistService,
        group_service: &dyn GroupService,
        entry_service: &dyn EntryService,
    ) -> Result<()> {
        let storage = Storage::new(playlist_service, group_service, entry_service);
        let file = File::create(path)?;
        serde_json::to_writer(file, &storage)?;
        Ok(())
    }

    fn read(
        &self,
        path: &str,
        playlist_service: &mut dyn PlaylistService,
        group_service: &mut dyn GroupService,
        entry_service: &mut dyn EntryService,
    ) -> Result<()> {
        let file = File::open(path)?;
        let storage: Storage = serde_json::from_reader(file)?;
        storage.apply(playlist_service, group_service, entry_service)
    }
}
