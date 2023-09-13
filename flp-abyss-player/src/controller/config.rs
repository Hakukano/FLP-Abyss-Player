#![allow(clippy::single_match)]

use std::sync::mpsc::Sender;

use crate::{
    controller::{Command, CommandName},
    library::differ::Differ,
    model::config::Config,
    view::{Packet, PacketName},
};

pub struct Controller {
    packet_tx: Sender<Packet>,
}

impl Controller {
    pub fn new(packet_tx: Sender<Packet>) -> Self {
        Self { packet_tx }
    }

    fn send_update_packet(&self) {
        self.packet_tx
            .send(Packet::new(
                PacketName::Update,
                serde_json::to_value(Config::all()).unwrap(),
            ))
            .unwrap();
    }
}

impl super::Controller for Controller {
    fn handle(&mut self, command: Command) {
        match command.name {
            CommandName::Update => {
                let mut config = Config::all();
                config.apply_diff(command.arguments);
                Config::set_all(config);
                self.send_update_packet();
            }
            _ => {}
        }
    }
}
