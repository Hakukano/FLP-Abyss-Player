use anyhow::Result;

use crate::{models::group::Group, utils::meta::MetaCmpBy};

mod fs;

pub trait GroupService: Send + Sync {
    fn all(&self) -> Vec<Group>;

    fn find_by_id(&self, id: &str) -> Option<Group> {
        self.all().iter().find(|group| group.id == id).cloned()
    }

    fn find_by_playlist_id(&self, playlist_id: &str) -> Vec<Group> {
        self.all()
            .iter()
            .filter(|group| group.playlist_id == playlist_id)
            .cloned()
            .collect()
    }

    fn save(&mut self, group: Group) -> Result<Group>;

    fn sort(&mut self, by: MetaCmpBy, ascend: bool);

    fn destroy(&mut self, id: &str) -> Result<Group>;
}

pub fn instantiate() -> Box<dyn GroupService> {
    Box::<fs::GroupService>::default()
}
