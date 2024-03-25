use std::{collections::HashMap, sync::mpsc::Sender};

use super::ChangeLocation;

pub struct View {
    change_location_tx: Sender<ChangeLocation>,

    initialized: bool,
}

impl View {
    pub fn new(change_location_tx: Sender<ChangeLocation>) -> Self {
        Self {
            change_location_tx,
            initialized: false,
        }
    }

    pub fn update(&mut self) {
        if !self.initialized {
            self.initialized = true;
            let _ = self.change_location_tx.send(ChangeLocation {
                path: vec!["configs".to_string(), "default".to_string()],
                query: HashMap::new(),
            });
        }
    }
}
