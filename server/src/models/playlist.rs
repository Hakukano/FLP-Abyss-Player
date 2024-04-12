use base64::{engine::general_purpose::URL_SAFE, Engine as _};
use flp_rusty_model::RustyModel;
use serde::{Deserialize, Serialize};

use super::group::Group;

#[derive(Clone, Deserialize, Serialize, RustyModel)]
#[rusty_model(service = "crate::services::playlist", has_many = ["group"])]
pub struct Playlist {
    pub id: String,
    pub name: String,
}

impl Playlist {
    pub fn new(name: String) -> Self {
        Self {
            id: URL_SAFE.encode(name.as_str()),
            name,
        }
    }
}
