use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::{
    services::{entry::EntryService, group::GroupService, playlist::PlaylistService},
    utils::meta::Meta,
};

use super::{entry::Entry, playlist::Playlist};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Group {
    pub id: String,
    pub meta: Meta,
    pub playlist_id: String,
}

impl Group {
    pub fn new(meta: Meta, playlist_id: String) -> Self {
        Self {
            id: meta.path.clone(),
            meta,
            playlist_id,
        }
    }

    pub fn save(self, group_service: &mut dyn GroupService) -> Result<Self> {
        group_service.save(self)
    }

    pub fn playlist(&self, playlist_service: &dyn PlaylistService) -> Option<Playlist> {
        playlist_service.find_by_id(self.playlist_id.as_str())
    }

    pub fn entries(&self, entry_service: &dyn EntryService) -> Vec<Entry> {
        entry_service.find_by_group_id(self.id.as_str())
    }
}
