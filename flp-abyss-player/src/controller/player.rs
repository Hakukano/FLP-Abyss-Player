use fuzzy_matcher::{skim::SkimMatcherV2, FuzzyMatcher};
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
                let paths = Player::playlist().body.item_paths;
                let matcher = SkimMatcherV2::default();
                let paths: Vec<(usize, String)> = paths
                    .iter()
                    .enumerate()
                    .filter_map(|(i, p)| {
                        matcher
                            .fuzzy_match(p.as_str(), search.as_str())
                            .map(|_| (i, p.clone()))
                    })
                    .collect();
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
