use anyhow::Result;
use std::collections::HashMap;

use crate::models::group::Group;

#[derive(Default)]
pub struct GroupService {
    data: HashMap<String, Group>,
}

impl super::GroupService for GroupService {
    fn all(&self) -> Vec<Group> {
        self.data.values().cloned().collect()
    }

    fn find_by_id(&self, id: &str) -> Option<Group> {
        self.data.get(id).cloned()
    }

    fn save(&mut self, group: Group) -> Result<Group> {
        self.data.insert(group.id.clone(), group.clone());
        Ok(group)
    }
}
