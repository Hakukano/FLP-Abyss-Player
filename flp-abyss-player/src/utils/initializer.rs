use crate::models::{config, player::Player, playlist::Playlist};

#[cfg(test)]
pub fn initialize() {
    let mut config = config::args::new();
    config.save();

    let mut playlist = Playlist::new("default".to_string(), &config);
    playlist.save();

    let mut player = Player::new("default".to_string(), &playlist);
    player.save();
}

#[cfg(not(test))]
pub fn initialize() {
    let mut config = config::args::new();
    config.save();

    let mut playlist = Playlist::new("default".to_string(), &config);
    playlist.save();

    let mut player = Player::new("default".to_string(), &playlist);
    player.save();
}
