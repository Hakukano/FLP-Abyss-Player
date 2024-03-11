use anyhow::Result;

use crate::models::playlist::Playlist;

mod fs;

pub trait PlaylistService: Send + Sync {
    fn all(&self) -> Vec<Playlist>;

    fn find_by_id(&self, id: &str) -> Option<Playlist> {
        self.all()
            .iter()
            .find(|playlist| playlist.id == id)
            .cloned()
    }

    fn save(&mut self, playlist: Playlist) -> Result<Playlist>;
}

pub fn instantiate() -> Box<dyn PlaylistService> {
    Box::<fs::PlaylistService>::default()
}
