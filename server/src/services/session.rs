use std::{fs::File, path::Path};

use anyhow::Result;

use crate::models::session::Session;

pub fn save(path: impl AsRef<Path>) -> Result<()> {
    let file = File::create(path)?;
    let session = Session::new();
    serde_json::to_writer(file, &session)?;
    Ok(())
}

pub fn load(path: impl AsRef<Path>) -> Result<()> {
    let file = File::open(path)?;
    let session: Session = serde_json::from_reader(file)?;
    session.apply()
}
