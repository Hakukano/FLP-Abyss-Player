#![allow(clippy::single_match)]

use eframe::egui::{
    Align, CentralPanel, Context, Frame, Layout, Margin, TextStyle::*, TopBottomPanel,
};
use serde_json::Value;
use std::sync::mpsc::Sender;

use crate::{
    controller::{Command, CommandName, ControllerType},
    library::{differ::Differ, fonts::gen_rich_text, helper::message_dialog_error},
    model::config::{Config, MediaType, VideoPlayer},
    view::{
        widget::{
            config::{
                media_type::ConfigMediaType, playlist_path::ConfigPlaylistPath,
                root_path::ConfigRootPath, video_player::ConfigVideoPlayer,
                video_player_path::ConfigVideoPlayerPath,
            },
            player_bar::PlayerBar,
        },
        Packet, PacketName,
    },
};

pub struct View {
    command_tx: Sender<Command>,

    state: Config,
    state_buffer: Config,

    can_play: bool,

    player_bar: PlayerBar,
    config_playlist_path: ConfigPlaylistPath,
    config_media_type: ConfigMediaType,
    config_root_path: ConfigRootPath,
    config_video_player: ConfigVideoPlayer,
    config_video_player_path: ConfigVideoPlayerPath,
}

impl View {
    pub fn new(config: Config, command_tx: Sender<Command>, ctx: &Context) -> Self {
        Self {
            command_tx,

            state: config.clone(),
            state_buffer: config,

            can_play: false,

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
        match packet.name {
            PacketName::Update => {
                let config: Config = serde_json::from_value(packet.data).unwrap();
                self.state = config;
                self.state_buffer = self.state.clone();
                self.can_play = self.state.can_play();
            }
            _ => {}
        }
    }

    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        TopBottomPanel::top("title")
            .frame(Frame::none().inner_margin(Margin {
                top: 10.0,
                bottom: 10.0,
                ..Default::default()
            }))
            .show(ctx, |ui| {
                ui.centered_and_justified(|ui| {
                    ui.label(gen_rich_text(ctx, t!("ui.config.title"), Heading, None))
                });
            });

        TopBottomPanel::bottom("go")
            .frame(Frame::none().inner_margin(Margin {
                top: 10.0,
                bottom: 10.0,
                right: 10.0,
                ..Default::default()
            }))
            .show(ctx, |ui| {
                ui.with_layout(Layout::right_to_left(Align::TOP), |ui| {
                    if ui
                        .button(gen_rich_text(ctx, t!("ui.config.go"), Button, None))
                        .clicked()
                    {
                        if self.can_play {
                            self.command_tx
                                .send(Command::new(
                                    ControllerType::Player,
                                    CommandName::Reload,
                                    Value::Null,
                                ))
                                .unwrap();
                        } else {
                            message_dialog_error("Please set all required fields!");
                        }
                    }
                });
            });

        CentralPanel::default()
            .frame(Frame::menu(ctx.style().as_ref()))
            .show(ctx, |ui| {
                ui.spacing_mut().item_spacing.y = 10.0;
                ui.with_layout(
                    Layout::left_to_right(Align::TOP).with_cross_justify(true),
                    |ui| {
                        let max_height = 20.0;
                        ui.set_height(max_height);
                        self.player_bar.show(
                            max_height,
                            ui,
                            &mut self.state_buffer.repeat,
                            &mut self.state_buffer.auto,
                            &mut self.state_buffer.auto_interval,
                            &mut self.state_buffer.lop,
                            &mut self.state_buffer.random,
                        );
                    },
                );

                ui.horizontal(|ui| {
                    self.config_playlist_path.show_config(
                        ui,
                        ctx,
                        &mut self.state_buffer.playlist_path,
                    );
                    self.config_playlist_path
                        .show_hint(ui, ctx, &self.state_buffer.playlist_path);
                });

                if self.state.playlist_path.is_none() {
                    ui.horizontal(|ui| {
                        self.config_media_type.show_config(
                            ui,
                            ctx,
                            &mut self.state_buffer.media_type,
                        );
                        self.config_media_type
                            .show_hint(ui, ctx, &self.state.media_type);
                    });

                    ui.horizontal(|ui| {
                        self.config_root_path.show_config(
                            ui,
                            ctx,
                            &mut self.state_buffer.root_path,
                        );
                        self.config_root_path
                            .show_hint(ui, ctx, &self.state_buffer.root_path);
                    });

                    if self.state.media_type == MediaType::Video {
                        ui.horizontal(|ui| {
                            self.config_video_player.show_config(
                                ui,
                                ctx,
                                &mut self.state_buffer.video_player,
                            );
                            self.config_video_player
                                .show_hint(ui, ctx, &self.state.video_player);
                        });

                        match self.state.video_player {
                            #[cfg(feature = "native")]
                            VideoPlayer::Native => {}
                            _ => {
                                ui.horizontal(|ui| {
                                    self.config_video_player_path.show_config(
                                        ui,
                                        ctx,
                                        &mut self.state_buffer.video_player_path,
                                    );
                                    self.config_video_player_path.show_hint(
                                        ui,
                                        ctx,
                                        &self.state_buffer.video_player_path,
                                    );
                                });
                            }
                        }
                    }
                }
            });

        // Cleanup phase
        if let Some(diff) = self.state.diff(&self.state_buffer) {
            self.command_tx
                .send(Command::new(
                    ControllerType::Config,
                    CommandName::Update,
                    diff,
                ))
                .unwrap();
        }
    }
}
