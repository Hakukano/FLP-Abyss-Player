use parking_lot::RwLock;

pub mod entry;
pub mod group;
pub mod playlist;

pub struct Services {
    entry: RwLock<Box<dyn entry::EntryService>>,
    group: RwLock<Box<dyn group::GroupService>>,
    playlist: RwLock<Box<dyn playlist::PlaylistService>>,
}

impl Services {
    pub fn new() -> Self {
        Self {
            entry: RwLock::new(entry::instantiate()),
            group: RwLock::new(group::instantiate()),
            playlist: RwLock::new(playlist::instantiate()),
        }
    }
}
