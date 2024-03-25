use crate::models::{config, player::Player, playlist::Playlist};

#[cfg(test)]
pub fn initialize() {
    let config = config::args::new();
    config.save();

    let playlist = Playlist::new("default".to_string(), &config);
    playlist.save();

    let player = Player::new("default".to_string(), &playlist);
    player.save();
}

#[cfg(not(test))]
pub fn initialize() {
    let config = config::args::new();
    config.save();

    let playlist = Playlist::new("default".to_string(), &config);
    playlist.save();

    let player = Player::new("default".to_string(), &playlist);
    player.save();
}
