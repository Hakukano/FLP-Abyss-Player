use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::sync::RwLock;

use crate::{library::playlist::Playlist, model::config::Config};

#[derive(Clone, Default, AccessibleModel, Deserialize, Serialize, Differ)]
#[accessible_model(singleton = PLAYER, rw_lock)]
pub struct Player {
    pub repeat: bool,
    pub auto: bool,
    pub auto_interval: u32,
    pub lop: bool,
    pub random: bool,

    pub playlist: Playlist,
}

impl Player {
    pub fn reload() {
        let config = Config::all();
        let mut lock = PLAYER.write().unwrap();
        lock.repeat = config.repeat;
        lock.auto = config.auto;
        lock.auto_interval = config.auto_interval;
        lock.lop = config.lop;
        lock.random = config.random;

        if let Some(playlist_path) = config.playlist_path.as_ref() {
            lock.playlist
                .load(playlist_path)
                .expect("Cannot load playlist");
        } else {
            lock.playlist.set_from_config(&config);
        }
    }
}

static PLAYER: Lazy<RwLock<Player>> = Lazy::new(|| RwLock::new(Player::default()));
