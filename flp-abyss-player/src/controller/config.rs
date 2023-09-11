use std::sync::mpsc::Sender;

use crate::{
    controller::{Command, COMMAND_NAME_INDEX},
    model::config::Config,
    view::{Packet, PACKET_NAME_INDEX},
};

pub struct Controller {
    packet_tx: Sender<Packet>,
}

impl Controller {
    pub fn new(packet_tx: Sender<Packet>) -> Self {
        Self { packet_tx }
    }
}

impl super::Controller for Controller {
    fn handle(&mut self, command: Command) {
        if command.name == COMMAND_NAME_INDEX {
            self.packet_tx
                .send(Packet::new(
                    PACKET_NAME_INDEX.to_string(),
                    serde_json::to_value(Config::all()).unwrap(),
                ))
                .expect("Cannot send packet");
        }
    }
}
