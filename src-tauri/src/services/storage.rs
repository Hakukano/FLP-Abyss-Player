use anyhow::Result;

use super::{entry::EntryService, group::GroupService, playlist::PlaylistService};

mod fs;

pub trait StorageService: Send + Sync {
    fn write(
        &self,
        path: &str,
        playlist_service: &dyn PlaylistService,
        group_service: &dyn GroupService,
        entry_service: &dyn EntryService,
    ) -> Result<()>;

    fn read(
        &self,
        path: &str,
        playlist_service: &mut dyn PlaylistService,
        group_service: &mut dyn GroupService,
        entry_service: &mut dyn EntryService,
    ) -> Result<()>;
}

pub fn instantiate() -> Box<dyn StorageService> {
    Box::<fs::StorageService>::default()
}
