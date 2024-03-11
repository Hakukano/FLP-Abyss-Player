use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::{
    services::{entry::EntryService, group::GroupService},
    utils::meta::Meta,
};

use super::group::Group;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Entry {
    pub id: String,
    pub meta: Meta,
    pub mime: String,
    pub group_id: Option<String>,
}

impl Entry {
    pub fn new(meta: Meta, mime: String, group_id: Option<String>) -> Self {
        Self {
            id: meta.path.clone(),
            mime,
            meta,
            group_id,
        }
    }

    pub fn save(self, entry_service: &mut EntryService) -> Result<Self> {
        entry_service.save(self)
    }

    pub fn group(&self, group_service: &GroupService) -> Option<Group> {
        self.group_id
            .as_ref()
            .and_then(|group_id| group_service.find_by_id(group_id))
    }

    pub fn matches_group(&self, group: impl AsRef<str>) -> bool {
        self.meta.path.starts_with(group.as_ref())
    }
}
