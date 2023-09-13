use eframe::{
    egui,
    egui::{Align, CentralPanel, Context, Frame, Layout, Margin, TextStyle::*, TopBottomPanel},
};
use serde_json::Value;
use std::{
    path::{Path, PathBuf},
    sync::{mpsc::Sender, Arc},
};

use crate::{
    controller::{Command, CommandName, ControllerType},
    library::fonts::gen_rich_text,
    model::{
        config::{Config, MediaType, VideoPlayer},
        player::Player,
    },
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
    widget::button_icon::ButtonIcon,
    CLI,
};

pub trait MediaPlayer: Send + Sync {
    fn is_loaded(&self) -> bool;
    fn is_end(&self) -> bool;
    fn reload(&mut self, path: &dyn AsRef<Path>, ctx: &egui::Context);
    fn sync(&mut self, paths: &[PathBuf]);
    fn show_central_panel(&mut self, ui: &mut egui::Ui, ctx: &egui::Context, can_input: bool);
}

pub struct View {
    command_tx: Sender<Command>,

    state: Option<Player>,
    state_buffer: Option<Player>,

    player_bar: PlayerBar,
    prev_icon: ButtonIcon,
    next_icon: ButtonIcon,
    playlist_icon: ButtonIcon,

    show_playlist: bool,
    home: bool,

    index: usize,
    next: bool,

    media_player: Box<dyn MediaPlayer>,
}

impl View {
    pub fn new(
        command_tx: Sender<Command>,
        ctx: &Context,
        #[cfg(feature = "native")] gl: Arc<glow::Context>,
    ) -> Self {
        let mut media_player: Box<dyn MediaPlayer> = match Config::media_type() {
            MediaType::Server => Box::new(server::MediaPlayer::new()),
            MediaType::Image => Box::new(image::MediaPlayer::new()),
            MediaType::Video => Box::new(video::MediaPlayer::new(
                ctx,
                #[cfg(feature = "native")]
                gl,
            )),
            _ => panic!("Unknown media type"),
        };
        let icon_path = Path::new(CLI.assets_path.as_str())
            .join("image")
            .join("icon");
        Self {
            command_tx,

            state: None,
            state_buffer: None,

            player_bar: PlayerBar::new(ctx),
            prev_icon: ButtonIcon::from_rgba_image_files("prev", icon_path.join("prev.png"), ctx),
            next_icon: ButtonIcon::from_rgba_image_files("next", icon_path.join("next.png"), ctx),
            playlist_icon: ButtonIcon::from_rgba_image_files(
                "playlist",
                icon_path.join("playlist.png"),
                ctx,
            ),

            show_playlist: false,
            home: false,

            index: 0,
            next: false,
            media_player,
        }
    }
}

impl super::View for View {
    fn handle(&mut self, packet: Packet) {
        match packet.name {
            PacketName::Update => {
                let config: Config = serde_json::from_value(packet.data).unwrap();
                self.can_play = config.can_play();
                self.state.replace(config.clone());
                self.state_buffer.replace(config);
            }
            _ => {}
        }
    }

    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        if let (Some(state), Some(state_buffer)) = (self.state.as_ref(), self.state_buffer.as_mut())
        {
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
                            && self.can_play
                        {
                            self.command_tx
                                .send(Command::new(
                                    ControllerType::Player,
                                    CommandName::Reload,
                                    Value::Null,
                                ))
                                .unwrap();
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
                                &mut state_buffer.repeat,
                                &mut state_buffer.auto,
                                &mut state_buffer.auto_interval,
                                &mut state_buffer.lop,
                                &mut state_buffer.random,
                            );
                        },
                    );

                    ui.horizontal(|ui| {
                        self.config_playlist_path.show_config(
                            ui,
                            ctx,
                            &mut state_buffer.playlist_path,
                        );
                        self.config_playlist_path
                            .show_hint(ui, ctx, &state.playlist_path);
                    });

                    if state.playlist_path.is_none() {
                        ui.horizontal(|ui| {
                            self.config_media_type.show_config(
                                ui,
                                ctx,
                                &mut state_buffer.media_type,
                            );
                            self.config_media_type.show_hint(ui, ctx, &state.media_type);
                        });

                        ui.horizontal(|ui| {
                            self.config_root_path
                                .show_config(ui, ctx, &mut state_buffer.root_path);
                            self.config_root_path.show_hint(ui, ctx, &state.root_path);
                        });

                        if state.media_type == MediaType::Video {
                            ui.horizontal(|ui| {
                                self.config_video_player.show_config(
                                    ui,
                                    ctx,
                                    &mut state_buffer.video_player,
                                );
                                self.config_video_player
                                    .show_hint(ui, ctx, &state.video_player);
                            });

                            match state.video_player {
                                #[cfg(feature = "native")]
                                VideoPlayer::Native => {}
                                _ => {
                                    ui.horizontal(|ui| {
                                        self.config_video_player_path.show_config(
                                            ui,
                                            ctx,
                                            &mut state_buffer.video_player_path,
                                        );
                                        self.config_video_player_path.show_hint(
                                            ui,
                                            ctx,
                                            &state.video_player_path,
                                        );
                                    });
                                }
                            }
                        }
                    }

                    ui.horizontal(|ui| {
                        if ui
                            .button(gen_rich_text(ctx, t!("ui.config.reset"), Button, None))
                            .clicked()
                        {
                            *state_buffer = state.clone();
                        }
                        if let Some(diff) = state.diff(state_buffer) {
                            if ui
                                .button(gen_rich_text(ctx, t!("ui.config.apply"), Button, None))
                                .clicked()
                            {
                                self.command_tx
                                    .send(Command::new(
                                        ControllerType::Config,
                                        CommandName::Update,
                                        diff,
                                    ))
                                    .unwrap();
                            }
                        } else {
                            ui.add_enabled(
                                false,
                                egui::Button::new(gen_rich_text(
                                    ctx,
                                    t!("ui.config.apply"),
                                    Button,
                                    None,
                                )),
                            );
                        }
                    });
                });
        } else {
            self.command_tx
                .send(Command::new(
                    ControllerType::Config,
                    CommandName::Read,
                    Value::Null,
                ))
                .expect("Cannot send command");
        }
    }
}
