use serde_json::Value;

use crate::{
    models::player::Player,
    views::{Method, Packet},
};

pub fn index() -> Packet {
    Packet::new(Method::Got, Player::all())
}

pub fn get(id: &str) -> Packet {
    Player::find(id)
        .map(|player| Packet::new(Method::Got, player))
        .unwrap_or_else(|| Packet::new(Method::Error, t!("player.error.not_found").as_ref()))
}

pub fn update(id: &str, diff: Value) -> Packet {
    if let Some(mut player) = Player::find(id) {
        player.apply_diff(diff);
        player.save();
        Packet::new(Method::Updated, ())
    } else {
        Packet::new(Method::Error, t!("player.error.not_found").as_ref())
    }
}
