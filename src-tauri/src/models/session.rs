use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::services::{entry::EntryService, group::GroupService, playlist::PlaylistService};

#[derive(Deserialize, Serialize)]
pub struct Session {
    pub playlists: Value,
    pub groups: Value,
    pub entries: Value,
}

impl Session {
    pub fn new(
        playlist_service: &dyn PlaylistService,
        group_service: &dyn GroupService,
        entry_service: &dyn EntryService,
    ) -> Self {
        Self {
            playlists: playlist_service.to_json(),
            groups: group_service.to_json(),
            entries: entry_service.to_json(),
        }
    }

    pub fn apply(
        self,
        playlist_service: &mut dyn PlaylistService,
        group_service: &mut dyn GroupService,
        entry_service: &mut dyn EntryService,
    ) -> Result<()> {
        playlist_service.set_json(self.playlists)?;
        group_service.set_json(self.groups)?;
        entry_service.set_json(self.entries)?;
        Ok(())
    }
}
