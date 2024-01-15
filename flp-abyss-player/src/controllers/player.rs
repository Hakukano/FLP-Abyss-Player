use std::sync::mpsc::Sender;

use crate::{
    controllers::{Command, CommandName},
    library::differ::Differ,
    models::player::Player,
    views::{Packet, PacketName, ViewType},
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
                        PacketName::ChangeView(ViewType::Player),
                        serde_json::to_value(Player::all()).unwrap(),
                    ))
                    .unwrap();
            }
            CommandName::Search => {
                let search: String = serde_json::from_value(command.arguments).unwrap();
                let paths = Player::playlist().filter(search);
                self.packet_tx
                    .send(Packet::new(
                        PacketName::Filter,
                        serde_json::to_value(paths).unwrap(),
                    ))
                    .unwrap();
            }
            CommandName::Save => {
                let path: String = serde_json::from_value(command.arguments).unwrap();
                let playlist = Player::playlist();
                playlist.save(path).unwrap();
            }
            CommandName::Load => {
                let path: String = serde_json::from_value(command.arguments).unwrap();
                let mut player = Player::all();
                player.playlist.load(path).unwrap();
                Player::set_all(player);
                self.send_update_packet();
            }
            _ => {}
        }
    }
}
