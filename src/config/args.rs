use crate::Cli;

pub fn new(cli: &Cli) -> super::Config {
    super::Config {
        media_type: cli.media_type,
        root_path: cli.root_path.clone(),
        repeat: cli.repeat,
        auto: cli.auto,
        auto_interval: cli.auto_interval,
        lop: cli.lop,
        random: cli.random,
        video_player: cli.video_player,
        video_player_path: cli.video_player_path.clone(),
    }
    .validate()
    .unwrap()
}
