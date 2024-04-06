use base64::{engine::general_purpose::URL_SAFE, Engine as _};
use serde::{Deserialize, Serialize};

use crate::utils::meta::Meta;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Entry {
    pub id: String,
    pub mime: String,
    pub meta: Meta,
    pub group_id: String,
}

impl Entry {
    pub fn new(meta: Meta, group_id: String) -> Self {
        Self {
            id: group_id.clone() + URL_SAFE.encode(meta.path.as_str()).as_str(),
            mime: mime_guess::from_path(meta.path.as_str())
                .first()
                .map(|mime| mime.to_string())
                .unwrap_or_default(),
            meta,
            group_id,
        }
    }
}
