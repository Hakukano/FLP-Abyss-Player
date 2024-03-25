use std::{collections::HashMap, sync::mpsc::Sender};

use eframe::egui::{
    Align, CentralPanel, Context, Frame, Layout, Margin, TextStyle::*, TopBottomPanel,
};
use serde_json::json;

use crate::{
    models::config::{Config, MediaType, VideoPlayer},
    utils::{fonts::gen_rich_text, helper::message_dialog_error},
};

use super::{
    widgets::config::{
        media_type::ConfigMediaType, playlist_path::ConfigPlaylistPath, root_path::ConfigRootPath,
        video_player::ConfigVideoPlayer, video_player_path::ConfigVideoPlayerPath,
    },
    ChangeLocation,
};

pub struct View {
    change_location_tx: Sender<Vec<String>>,

    config: Config,

    config_playlist_path: ConfigPlaylistPath,
    config_media_type: ConfigMediaType,
    config_root_path: ConfigRootPath,
    config_video_player: ConfigVideoPlayer,
    config_video_player_path: ConfigVideoPlayerPath,
}

impl View {
    pub fn new(id: &str, change_location_tx: Sender<Vec<String>>, ctx: &Context) -> Self {
        Self {
            change_location_tx,

            config: Config::find(id).expect("Config not found"),

            config_playlist_path: ConfigPlaylistPath::new(ctx),
            config_media_type: ConfigMediaType::new(ctx),
            config_root_path: ConfigRootPath::new(ctx),
            config_video_player: ConfigVideoPlayer::new(ctx),
            config_video_player_path: ConfigVideoPlayerPath::new(ctx),
        }
    }

    pub fn update(&mut self, ctx: &Context) {
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
                        if self.config.can_play() {
                            self.change_location_tx
                                .send(ChangeLocation {
                                    path: vec!["players".to_string(), "default".to_string()],
                                    query: HashMap::from_iter([
                                        "playlist_id".to_string(),
                                        json!("default"),
                                    ]),
                                })
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

                ui.horizontal(|ui| {
                    self.config_playlist_path
                        .show_config(ui, ctx, &mut self.config.playlist_path);
                    self.config_playlist_path
                        .show_hint(ui, ctx, &self.config.playlist_path);
                });

                if self.config.playlist_path.is_none() {
                    ui.horizontal(|ui| {
                        self.config_media_type
                            .show_config(ui, ctx, &mut self.config.media_type);
                        self.config_media_type
                            .show_hint(ui, ctx, &self.config.media_type);
                    });

                    ui.horizontal(|ui| {
                        self.config_root_path
                            .show_config(ui, ctx, &mut self.config.root_path);
                        self.config_root_path
                            .show_hint(ui, ctx, &self.config.root_path);
                    });

                    if self.config.media_type == MediaType::Video {
                        ui.horizontal(|ui| {
                            self.config_video_player.show_config(
                                ui,
                                ctx,
                                &mut self.config.video_player,
                            );
                            self.config_video_player
                                .show_hint(ui, ctx, &self.config.video_player);
                        });

                        match self.config.video_player {
                            VideoPlayer::Native => {}
                            _ => {
                                ui.horizontal(|ui| {
                                    self.config_video_player_path.show_config(
                                        ui,
                                        ctx,
                                        &mut self.config.video_player_path,
                                    );
                                    self.config_video_player_path.show_hint(
                                        ui,
                                        ctx,
                                        &self.config.video_player_path,
                                    );
                                });
                            }
                        }
                    }
                }
            });

        // Cleanup phase
        self.config.save();
    }
}
