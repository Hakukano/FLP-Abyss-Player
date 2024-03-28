use anyhow::Result;

use super::{entry::EntryService, group::GroupService, playlist::PlaylistService};

mod fs;

pub trait SessionService: Send + Sync {
    fn save(
        &self,
        path: &str,
        playlist_service: &dyn PlaylistService,
        group_service: &dyn GroupService,
        entry_service: &dyn EntryService,
    ) -> Result<()>;

    fn load(
        &self,
        path: &str,
        playlist_service: &mut dyn PlaylistService,
        group_service: &mut dyn GroupService,
        entry_service: &mut dyn EntryService,
    ) -> Result<()>;
}

pub fn instantiate() -> Box<dyn SessionService> {
    Box::<fs::SessionService>::default()
}
