use parking_lot::RwLock;

pub mod app_config;
pub mod entry;
pub mod group;
pub mod playlist;
pub mod session;

pub struct Services {
    pub app_config: RwLock<Box<dyn app_config::AppConfigService>>,
    pub entry: RwLock<Box<dyn entry::EntryService>>,
    pub group: RwLock<Box<dyn group::GroupService>>,
    pub playlist: RwLock<Box<dyn playlist::PlaylistService>>,
    pub session: RwLock<Box<dyn session::SessionService>>,
}

impl Services {
    pub fn new() -> Self {
        Self {
            app_config: RwLock::new(app_config::instantiate()),
            entry: RwLock::new(entry::instantiate()),
            group: RwLock::new(group::instantiate()),
            playlist: RwLock::new(playlist::instantiate()),
            session: RwLock::new(session::instantiate()),
        }
    }
}