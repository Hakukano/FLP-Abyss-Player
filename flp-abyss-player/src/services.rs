use std::sync::Arc;

use parking_lot::RwLock;

pub mod app_config;
pub mod entry;
pub mod group;
pub mod playlist;
pub mod session;

#[derive(Clone)]
pub struct Services {
    pub app_config: Arc<RwLock<Box<dyn app_config::AppConfigService>>>,
    pub entry: Arc<RwLock<Box<dyn entry::EntryService>>>,
    pub group: Arc<RwLock<Box<dyn group::GroupService>>>,
    pub playlist: Arc<RwLock<Box<dyn playlist::PlaylistService>>>,
    pub session: Arc<RwLock<Box<dyn session::SessionService>>>,
}

impl Services {
    pub fn new() -> Self {
        Self {
            app_config: Arc::new(RwLock::new(app_config::instantiate())),
            entry: Arc::new(RwLock::new(entry::instantiate())),
            group: Arc::new(RwLock::new(group::instantiate())),
            playlist: Arc::new(RwLock::new(playlist::instantiate())),
            session: Arc::new(RwLock::new(session::instantiate())),
        }
    }
}
