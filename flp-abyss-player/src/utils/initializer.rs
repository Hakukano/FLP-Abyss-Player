use crate::models::{config, player::Player, playlist::Playlist};

#[cfg(test)]
pub fn initialize() {
    let config = config::args::new();
    config.save();

    rust_i18n::set_locale(config.locale.as_str());

    let playlist = Playlist::new("default".to_string(), &config);
    playlist.save();

    let player = Player::new("default".to_string(), &playlist);
    player.save();
}

#[cfg(not(test))]
pub fn initialize() {
    let config = config::args::new();
    config.save();

    rust_i18n::set_locale(config.locale.as_str());

    let playlist = Playlist::new("default".to_string(), &config);
    playlist.save();

    let player = Player::new("default".to_string(), &playlist);
    player.save();
}
