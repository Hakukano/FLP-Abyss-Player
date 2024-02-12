use crate::models::playlist::entry::Entry;
use anyhow::Result;

use super::group::Group;

#[derive(Default)]
pub struct Playlist {
    groups: Vec<Group>,
}

impl super::Playlist for Playlist {
    fn scan(&self, root_path: String, allowed_mimes: Vec<String>) -> Result<Vec<Entry>> {
        todo!()
    }

    fn create_groups(&mut self, paths: Vec<String>) -> Result<()> {
        todo!()
    }

    fn groups(&self) -> &Vec<Group> {
        &self.groups
    }

    fn groups_mut(&mut self) -> &mut Vec<Group> {
        &mut self.groups
    }
}
