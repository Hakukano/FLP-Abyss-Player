use std::fs::File;

use anyhow::Result;

use crate::{
    models::session::Session,
    services::{entry::EntryService, group::GroupService, playlist::PlaylistService},
};

#[derive(Default)]
pub struct SessionService {}

impl super::SessionService for SessionService {
    fn save(
        &self,
        path: &str,
        playlist_service: &dyn PlaylistService,
        group_service: &dyn GroupService,
        entry_service: &dyn EntryService,
    ) -> Result<()> {
        let session = Session::new(playlist_service, group_service, entry_service);
        let file = File::create(path)?;
        serde_json::to_writer(file, &session)?;
        Ok(())
    }

    fn load(
        &self,
        path: &str,
        playlist_service: &mut dyn PlaylistService,
        group_service: &mut dyn GroupService,
        entry_service: &mut dyn EntryService,
    ) -> Result<()> {
        let file = File::open(path)?;
        let session: Session = serde_json::from_reader(file)?;
        session.apply(playlist_service, group_service, entry_service)
    }
}
