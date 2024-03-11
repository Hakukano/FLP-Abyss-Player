use anyhow::Result;

use crate::{models::group::Group, utils::meta::MetaCmpBy};

#[derive(Default)]
pub struct GroupService {
    data: Vec<Group>,
}

impl super::GroupService for GroupService {
    fn all(&self) -> Vec<Group> {
        self.data.clone()
    }

    fn save(&mut self, group: Group) -> Result<Group> {
        if let Some(origin) = self.data.iter_mut().find(|g| g.id == group.id) {
            *origin = group.clone();
        } else {
            self.data.push(group.clone());
        }
        Ok(group)
    }

    fn sort(&mut self, by: MetaCmpBy, ascend: bool) {
        self.data.sort_by(|a, b| a.meta.cmp_by(&b.meta, by, ascend));
    }
}
