use anyhow::Result;
use base64::{engine::general_purpose::URL_SAFE, Engine as _};
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
    pub group_id: String,
}

impl Entry {
    pub fn new(meta: Meta, mime: String, group_id: String) -> Self {
        Self {
            id: group_id.clone() + URL_SAFE.encode(meta.path.as_str()).as_str(),
            mime,
            meta,
            group_id,
        }
    }

    pub fn save(self, entry_service: &mut dyn EntryService) -> Result<Self> {
        entry_service.save(self)
    }

    pub fn group(&self, group_service: &dyn GroupService) -> Option<Group> {
        group_service.find_by_id(self.group_id.as_str())
    }

    pub fn matches_group(&self, group: impl AsRef<str>) -> bool {
        self.meta.path.starts_with(group.as_ref())
    }
}
