use anyhow::Result;
use base64::{engine::general_purpose::URL_SAFE, Engine as _};
use serde::{Deserialize, Serialize};

use crate::services::playlist::PlaylistService;

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
}
