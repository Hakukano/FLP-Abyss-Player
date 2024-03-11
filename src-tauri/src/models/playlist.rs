use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::{
    services::{group::GroupService, playlist::PlaylistService},
    utils::meta::Meta,
};

use super::group::Group;

#[derive(Clone, Deserialize, Serialize)]
pub struct Playlist {
    pub id: String,
    pub meta: Meta,
}

impl Playlist {
    pub fn new(meta: Meta) -> Self {
        Self {
            id: meta.path.clone(),
            meta,
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
