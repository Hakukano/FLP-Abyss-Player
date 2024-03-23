use once_cell::sync::Lazy;
use parking_lot::RwLock;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::{playlist::Playlist, Singleton};

#[derive(Clone, Default, StaticRecord, Deserialize, Serialize, Differ)]
#[static_record(singleton = SINGLETON, belongs_to = ["playlist"])]
pub struct Player {
    pub id: String,

    pub repeat: bool,
    pub auto: bool,
    pub auto_interval: u32,
    pub lop: bool,
    pub random: bool,

    pub index: usize,

    pub playlist_id: String,
}

impl Player {
    pub fn new(id: String, playlist: &Playlist) -> Self {
        let config = playlist.config().expect("Config not found");

        Self {
            id,
            repeat: config.repeat,
            auto: config.auto,
            auto_interval: config.auto_interval,
            lop: config.lop,
            random: config.random,
            index: 0,
            playlist_id: playlist.id.clone(),
        }
    }

    pub fn random_next(&mut self) {
        let mut rng = rand::thread_rng();
        self.index = rng.gen_range(0..self.item_count());
    }

    pub fn next(&mut self) {
        if self.repeat {
            return;
        }
        if self.random {
            return self.random_next();
        }
        if self.index == self.item_count() - 1 && self.lop {
            self.index = 0;
        } else if self.index < self.item_count() - 1 {
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
            self.index = self.item_count() - 1;
        } else if self.index > 0 {
            self.index -= 1;
        }
    }

    fn item_count(&self) -> usize {
        self.playlist()
            .map(|playlist| playlist.item_paths().len())
            .unwrap_or(0)
    }
}

static SINGLETON: Singleton<Player> = Lazy::new(RwLock::default);
