use once_cell::sync::Lazy;
use parking_lot::RwLock;
use rand::Rng;
use serde::{Deserialize, Serialize};

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
    pub index: usize,
}

impl Player {
    pub fn reload() {
        let config = Config::all();
        let mut lock = PLAYER.write();
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
        lock.index = 0;
    }

    pub fn item_paths(&self) -> &[String] {
        self.playlist.body.item_paths.as_slice()
    }

    pub fn random_next(&mut self) {
        let mut rng = rand::thread_rng();
        self.index = rng.gen_range(0..self.item_paths().len());
    }

    pub fn next(&mut self) {
        if self.repeat {
            return;
        }
        if self.random {
            return self.random_next();
        }
        if self.index == self.item_paths().len() - 1 && self.lop {
            self.index = 0;
        } else if self.index < self.item_paths().len() - 1 {
            self.index += 1;
        }
    }

    pub fn prev(&mut self) {
        if self.repeat {
            return;
        }
        if self.random {
            return self.random_next();
        }
        if self.index == 0 && self.lop {
            self.index = self.item_paths().len() - 1;
        } else if self.index > 0 {
            self.index -= 1;
        }
    }
}

static PLAYER: Lazy<RwLock<Player>> = Lazy::new(|| RwLock::new(Player::default()));
