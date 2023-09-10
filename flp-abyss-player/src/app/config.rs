use eframe::{
    egui::{self, style::Margin, Frame, Layout, TextStyle::*},
    emath::Align,
};

use crate::{
    library::{fonts::gen_rich_text, helper::message_dialog_error, playlist},
    model::config::*,
    widget::{
        config::{
            media_type::ConfigMediaType, playlist_path::ConfigPlaylistPath,
            root_path::ConfigRootPath, video_player::ConfigVideoPlayer,
            video_player_path::ConfigVideoPlayerPath,
        },
        player_bar::PlayerBar,
    },
    CLI,
};

pub struct State {
    player_bar: PlayerBar,
    config_playlist_path: ConfigPlaylistPath,
    config_media_type: ConfigMediaType,
    config_root_path: ConfigRootPath,
    config_video_player: ConfigVideoPlayer,
    config_video_player_path: ConfigVideoPlayerPath,

    go: bool,

    pub playlist: Option<(playlist::Header, playlist::Body)>,
}

impl State {
    pub fn new(ctx: &egui::Context) -> Self {
        Self {
            player_bar: PlayerBar::new(ctx),
            config_playlist_path: ConfigPlaylistPath::new(ctx),
            config_media_type: ConfigMediaType::new(ctx),
            config_root_path: ConfigRootPath::new(ctx),
            config_video_player: ConfigVideoPlayer::new(ctx),
            config_video_player_path: ConfigVideoPlayerPath::new(ctx),

            go: false,

            playlist: None,
        }
    }

    pub fn should_go(&self) -> bool {
        self.go
    }

    fn try_go(&mut self) -> bool {
        let path_set = Config::root_path().is_some();
        let other_set = match Config::media_type() {
            MediaType::Server => true,
            MediaType::Image => true,
            MediaType::Video => match Config::video_player() {
                VideoPlayer::Unset => false,
                #[cfg(feature = "native")]
                VideoPlayer::Native => true,
                _ => Config::video_player_path().is_some(),
            },
            _ => false,
        };
        if let Some(playlist_path) = Config::playlist_path() {
            match playlist::Header::load(playlist_path) {
                Err(err) => {
                    message_dialog_error(err.to_string());
                    false
                }
                Ok((rest, header)) => match playlist::Body::load(rest) {
                    Err(err) => {
                        message_dialog_error(err.to_string());
                        false
                    }
                    Ok(body) => {
                        header.writer_config();
                        self.playlist.replace((header, body));
                        self.go = true;
                        true
                    }
                },
            }
        } else if path_set && other_set {
            self.go = true;
            true
        } else {
            message_dialog_error(t!("ui.config.alert"));
            false
        }
    }

    pub fn update(&mut self, ctx: &egui::Context) {
        if CLI.playlist_path.is_some() && Config::playlist_path().is_some() && !self.try_go() {
            panic!("Broken playlist file");
        }

        egui::TopBottomPanel::top("title")
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

        egui::TopBottomPanel::bottom("go")
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
                        self.try_go();
                    }
                });
            });

        egui::CentralPanel::default()
            .frame(Frame::none().inner_margin(Margin {
                top: 10.0,
                bottom: 10.0,
                right: 10.0,
                left: 10.0,
            }))
            .show(ctx, |ui| {
                ui.spacing_mut().item_spacing.y = 10.0;
                ui.with_layout(
                    Layout::left_to_right(Align::TOP).with_cross_justify(true),
                    |ui| {
                        let max_height = 20.0;
                        ui.set_height(max_height);
                        self.player_bar.show(max_height, ui);
                    },
                );

                ui.horizontal(|ui| {
                    self.config_playlist_path.show_config(ui, ctx);
                    self.config_playlist_path.show_hint(ui, ctx);
                });

                if Config::playlist_path().is_none() {
                    ui.horizontal(|ui| {
                        self.config_media_type.show_config(ui, ctx);
                        self.config_media_type.show_hint(ui, ctx);
                    });

                    ui.horizontal(|ui| {
                        self.config_root_path.show_config(ui, ctx);
                        self.config_root_path.show_hint(ui, ctx);
                    });

                    if Config::media_type() == MediaType::Video {
                        ui.horizontal(|ui| {
                            self.config_video_player.show_config(ui, ctx);
                            self.config_video_player.show_hint(ui, ctx);
                        });

                        match Config::video_player() {
                            #[cfg(feature = "native")]
                            VideoPlayer::Native => {}
                            _ => {
                                ui.horizontal(|ui| {
                                    self.config_video_player_path.show_config(ui, ctx);
                                    self.config_video_player_path.show_hint(ui, ctx);
                                });
                            }
                        }
                    }
                }
            });
    }
}
