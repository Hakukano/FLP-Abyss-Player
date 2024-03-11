use anyhow::{anyhow, Result};
use std::collections::HashMap;

use crate::models::playlist::Playlist;

#[derive(Default)]
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

    fn destroy(&mut self, id: &str) -> Result<Playlist> {
        self.data
            .remove(id)
            .ok_or_else(|| anyhow!("Playlist not found"))
    }
}
