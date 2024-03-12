use anyhow::Result;
use base64::{engine::general_purpose::URL_SAFE, Engine as _};
use serde::{Deserialize, Serialize};

use crate::services::{group::GroupService, playlist::PlaylistService};

use super::group::Group;

#[derive(Clone, Deserialize, Serialize)]
pub struct Playlist {
    pub id: String,
    pub name: String,
}

impl Playlist {
    pub fn new(name: String) -> Self {
        Self {
            id: URL_SAFE.encode(name.as_str()),
            name,
        }
    }

    pub fn save(self, playlist_service: &mut dyn PlaylistService) -> Result<Self> {
        playlist_service.save(self)
    }

    pub fn destroy(self, playlist_service: &mut dyn PlaylistService) -> Result<Self> {
        playlist_service.destroy(self.id.as_str())
    }

    pub fn groups(&self, group_service: &dyn GroupService) -> Vec<Group> {
        group_service.find_by_playlist_id(self.id.as_str())
    }
}
