use crate::Cli;

pub fn new(cli: &Cli) -> super::Config {
    super::Config {
        repeat: cli.repeat,
        auto: cli.auto,
        auto_interval: cli.auto_interval,
        lop: cli.lop,

        playlist_path: cli.playlist_path.clone(),

        media_type: cli.media_type,
        root_path: cli.root_path.clone(),
        random: cli.random,
        video_player: cli.video_player,
        video_player_path: cli.video_player_path.clone(),
    }
    .validate()
    .unwrap()
}
