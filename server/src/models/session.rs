use std::path::Path;

use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::services::{entry, group, playlist, session};

#[derive(Deserialize, Serialize)]
pub struct Session {
    pub playlists: Value,
    pub groups: Value,
    pub entries: Value,
}

impl Session {
    pub fn new() -> Self {
        Self {
            playlists: playlist::to_json(),
            groups: group::to_json(),
            entries: entry::to_json(),
        }
    }

    pub fn apply(self) -> Result<()> {
        playlist::set_json(self.playlists)?;
        group::set_json(self.groups)?;
        entry::set_json(self.entries)?;
        Ok(())
    }

    pub fn save(path: impl AsRef<Path>) -> Result<()> {
        session::save(path)
    }

    pub fn load(path: impl AsRef<Path>) -> Result<()> {
        session::load(path)
    }
}
