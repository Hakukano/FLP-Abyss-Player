use eframe::{egui::Context, Frame};
use serde_json::Value;
use std::sync::mpsc::Sender;

use crate::{
    controller::{Command, ControllerType, COMMAND_NAME_INDEX},
    model::config::Config,
    view::{
        widget::{
            config::{
                media_type::ConfigMediaType, playlist_path::ConfigPlaylistPath,
                root_path::ConfigRootPath, video_player::ConfigVideoPlayer,
                video_player_path::ConfigVideoPlayerPath,
            },
            player_bar::PlayerBar,
        },
        Packet, PACKET_NAME_INDEX,
    },
};

pub struct View {
    packet_tx: Sender<Packet>,
    command_tx: Sender<Command>,

    state: Option<Config>,
    state_buffer: Option<Config>,

    player_bar: PlayerBar,
    config_playlist_path: ConfigPlaylistPath,
    config_media_type: ConfigMediaType,
    config_root_path: ConfigRootPath,
    config_video_player: ConfigVideoPlayer,
    config_video_player_path: ConfigVideoPlayerPath,
}

impl View {
    pub fn new(packet_tx: Sender<Packet>, command_tx: Sender<Command>, ctx: &Context) -> Self {
        Self {
            packet_tx,
            command_tx,

            state: None,
            state_buffer: None,

            player_bar: PlayerBar::new(ctx),
            config_playlist_path: ConfigPlaylistPath::new(ctx),
            config_media_type: ConfigMediaType::new(ctx),
            config_root_path: ConfigRootPath::new(ctx),
            config_video_player: ConfigVideoPlayer::new(ctx),
            config_video_player_path: ConfigVideoPlayerPath::new(ctx),
        }
    }
}

impl super::View for View {
    fn handle(&mut self, packet: Packet) {
        if packet.name == PACKET_NAME_INDEX && (self.state.is_none() || self.state_buffer.is_none())
        {
            let state: Config = serde_json::from_value(packet.data).unwrap();
            self.state.replace(state.clone());
            self.state_buffer.replace(state);
        }
    }

    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        if let (Some(state), Some(state_buffer)) = (self.state.as_mut(), self.state_buffer.as_mut())
        {
        } else {
            self.command_tx
                .send(Command::new(
                    ControllerType::Config,
                    COMMAND_NAME_INDEX.to_string(),
                    Value::Null,
                ))
                .expect("Cannot send command");
        }
    }
}
