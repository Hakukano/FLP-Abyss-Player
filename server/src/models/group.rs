use base64::{engine::general_purpose::URL_SAFE, Engine as _};
use flp_rusty_model::RustyModel;
use serde::{Deserialize, Serialize};

use super::{entry::Entry, playlist::Playlist};
use crate::utils::meta::Meta;

#[derive(Clone, Debug, Deserialize, Serialize, RustyModel)]
#[rusty_model(service = "crate::services::group", belongs_to = ["playlist"], has_many = ["entry"])]
pub struct Group {
    pub id: String,
    pub meta: Meta,
    #[rusty_model(findable)]
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
}
