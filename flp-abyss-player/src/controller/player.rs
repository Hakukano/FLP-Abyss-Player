use std::sync::mpsc::Sender;

use crate::{
    controller::{Command, CommandName},
    model::player::Player,
    view::{Packet, PacketName, ViewType},
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
                serde_json::to_value(Player::all()).unwrap(),
            ))
            .unwrap();
    }
}

impl super::Controller for Controller {
    fn handle(&mut self, command: Command) {
        match command.name {
            CommandName::Read => {
                self.send_update_packet();
            }
            CommandName::Update => {
                let mut player = Player::all();
                player.apply_diff(command.arguments);
                Player::set_all(player);
                self.send_update_packet();
            }
            CommandName::Reload => {
                Player::reload();
                self.packet_tx
                    .send(Packet::new(
                        PacketName::ChangeView,
                        serde_json::to_value(ViewType::Player).unwrap(),
                    ))
                    .unwrap();
            }
            _ => {}
        }
    }
}
