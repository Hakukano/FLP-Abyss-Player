use crate::Cli;

pub fn new(cli: &Cli) -> super::Config {
    super::Config {
        media_type: cli.media_type.clone().unwrap_or_default(),
        root_path: cli.root_path.clone(),
        video_player: cli.video_player.clone().unwrap_or_default(),
    }
}
