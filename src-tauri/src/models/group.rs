use anyhow::Result;
use base64::{engine::general_purpose::URL_SAFE, Engine as _};
use serde::{Deserialize, Serialize};

use crate::{services::group::GroupService, utils::meta::Meta};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Group {
    pub id: String,
    pub meta: Meta,
    pub playlist_id: String,
}

impl Group {
    pub fn new(meta: Meta, playlist_id: String) -> Self {
        Self {
            id: playlist_id.clone() + URL_SAFE.encode(meta.path.as_str()).as_str(),
            meta,
            playlist_id,
        }
    }

    pub fn save(self, group_service: &mut dyn GroupService) -> Result<Self> {
        group_service.save(self)
    }
}
