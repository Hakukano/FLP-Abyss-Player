use base64::{engine::general_purpose::URL_SAFE, Engine as _};
use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
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
