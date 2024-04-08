use anyhow::Result;
use serde_json::Value;

use crate::models::playlist::Playlist;

use super::{entry::EntryService, group::GroupService};

mod memory;

pub trait PlaylistService: Send + Sync {
    fn all(&self) -> Vec<Playlist>;

    fn find_by_id(&self, id: &str) -> Option<Playlist> {
        self.all()
            .iter()
            .find(|playlist| playlist.id == id)
            .cloned()
    }

    fn save(&mut self, playlist: Playlist) -> Result<Playlist>;

    fn destroy(
        &mut self,
        id: &str,
        group_service: &mut dyn GroupService,
        entry_service: &mut dyn EntryService,
    ) -> Result<Playlist>;

    fn to_json(&self) -> Value;

    fn set_json(&mut self, value: Value) -> Result<()>;
}

pub fn instantiate() -> Box<dyn PlaylistService> {
    Box::<memory::PlaylistService>::default()
}
