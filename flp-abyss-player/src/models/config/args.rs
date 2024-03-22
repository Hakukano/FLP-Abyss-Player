use crate::CLI;

pub fn new() -> super::Config {
    super::Config {
        id: "default".to_string(),
        locale: if let Some(locale) = CLI.locale.as_ref() {
            locale.clone()
        } else {
            sys_locale::get_locale()
                .map(|l| l.replace('-', "_"))
                .unwrap_or_else(|| "en_US".to_string())
        },

        repeat: CLI.repeat,
        auto: CLI.auto,
        auto_interval: CLI.auto_interval,
        lop: CLI.lop,
        random: CLI.random,

        playlist_path: CLI.playlist_path.clone(),

        media_type: CLI.media_type,
        root_path: CLI.root_path.clone(),
        video_player: CLI.video_player,
        video_player_path: CLI.video_player_path.clone(),
    }
}
