use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

use crate::{
    models::playlist::Playlist,
    services::{entry::EntryService, group::GroupService},
};

#[derive(Default, Deserialize, Serialize)]
pub struct PlaylistService {
    data: HashMap<String, Playlist>,
}

impl super::PlaylistService for PlaylistService {
    fn all(&self) -> Vec<Playlist> {
        self.data.values().cloned().collect()
    }

    fn find_by_id(&self, id: &str) -> Option<Playlist> {
        self.data.get(id).cloned()
    }

    fn save(&mut self, playlist: Playlist) -> Result<Playlist> {
        self.data.insert(playlist.id.clone(), playlist.clone());
        Ok(playlist)
    }

    fn destroy(
        &mut self,
        id: &str,
        group_service: &mut dyn GroupService,
        entry_service: &mut dyn EntryService,
    ) -> Result<Playlist> {
        group_service
            .find_by_playlist_id(id)
            .into_iter()
            .map(|group| group_service.destroy(group.id.as_str(), entry_service))
            .collect::<Result<Vec<_>, _>>()?;

        self.data
            .remove(id)
            .ok_or_else(|| anyhow!("Playlist not found"))
    }

    fn to_json(&self) -> Value {
        serde_json::to_value(self).expect("Corrupted playlist data")
    }

    fn set_json(&mut self, value: Value) -> Result<()> {
        *self = serde_json::from_value(value)?;
        Ok(())
    }
}
