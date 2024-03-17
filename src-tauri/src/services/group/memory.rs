use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{models::group::Group, services::entry::EntryService, utils::meta::MetaCmpBy};

#[derive(Deserialize, Serialize)]
pub struct GroupService {
    data: Vec<Group>,
    last_sort_by: MetaCmpBy,
    last_sort_ascend: bool,
}

impl Default for GroupService {
    fn default() -> Self {
        Self {
            data: Vec::new(),
            last_sort_by: MetaCmpBy::Path,
            last_sort_ascend: true,
        }
    }
}

impl super::GroupService for GroupService {
    fn all(&self) -> Vec<Group> {
        self.data.clone()
    }

    fn set_all(&mut self, groups: Vec<Group>) {
        self.data = groups;
        self.last_sort_by = MetaCmpBy::Default;
        self.last_sort_ascend = true;
    }

    fn save(&mut self, group: Group) -> Result<Group> {
        if let Some(origin) = self.data.iter_mut().find(|g| g.id == group.id) {
            *origin = group.clone();
        } else {
            self.data.push(group.clone());
            self.sort(self.last_sort_by, self.last_sort_ascend);
        }
        Ok(group)
    }

    fn sort(&mut self, by: MetaCmpBy, ascend: bool) {
        self.data.sort_by(|a, b| a.meta.cmp_by(&b.meta, by, ascend));
        self.last_sort_by = by;
        self.last_sort_ascend = ascend;
    }

    fn destroy(&mut self, id: &str, entry_service: &mut dyn EntryService) -> Result<Group> {
        entry_service
            .find_by_group_id(id)
            .into_iter()
            .map(|entry| entry_service.destroy(entry.id.as_str()))
            .collect::<Result<Vec<_>, _>>()?;

        let (index, _) = self
            .data
            .iter()
            .enumerate()
            .find(|(_, g)| g.id == id)
            .ok_or_else(|| anyhow!("Playlist not found"))?;
        Ok(self.data.remove(index))
    }

    fn to_json(&self) -> Value {
        serde_json::to_value(self).expect("Corrupted playlist data")
    }

    fn set_json(&mut self, value: Value) -> Result<()> {
        *self = serde_json::from_value(value)?;
        Ok(())
    }
}
