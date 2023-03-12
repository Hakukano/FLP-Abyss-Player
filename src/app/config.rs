use eframe::{
    egui::{self, style::Margin, Frame, Layout, TextStyle::*},
    emath::Align,
};

use crate::{
    config::*,
    font::gen_rich_text,
    get_cli,
    helper::message_dialog_error,
    locale, playlist,
    widget::{
        config::{
            media_type::ConfigMediaType, playlist_path::ConfigPlaylistPath,
            root_path::ConfigRootPath, video_player::ConfigVideoPlayer,
            video_player_path::ConfigVideoPlayerPath,
        },
        player_bar::PlayerBar,
    },
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
        let cli = get_cli();
        Self {
            player_bar: PlayerBar::new(ctx),
            config_playlist_path: ConfigPlaylistPath::new(ctx, cli),
            config_media_type: ConfigMediaType::new(ctx, cli),
            config_root_path: ConfigRootPath::new(ctx, cli),
            config_video_player: ConfigVideoPlayer::new(ctx, cli),
            config_video_player_path: ConfigVideoPlayerPath::new(ctx, cli),

            go: false,

            playlist: None,
        }
    }

    pub fn should_go(&self) -> bool {
        self.go
    }

    fn try_go(&mut self, config: &mut Config, locale: &locale::ui::Config) -> bool {
        let path_set = config.root_path.is_some();
        let other_set = match config.media_type {
            MediaType::Image => true,
            MediaType::Video => match config.video_player {
                VideoPlayer::Unset => false,
                #[cfg(feature = "native")]
                VideoPlayer::Native => true,
                _ => config.video_player_path.is_some(),
            },
            _ => false,
        };
        if let Some(playlist_path) = config.playlist_path.clone() {
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
                        header.writer_config(config);
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
            message_dialog_error(locale.alert.clone());
            false
        }
    }

    pub fn update(&mut self, ctx: &egui::Context) {
        let mut config = get().write().expect("Cannot get config lock");
        let locale = &locale::get().ui.config;

        if get_cli().playlist_path.is_some()
            && config.playlist_path.is_some()
            && !self.try_go(&mut config, locale)
        {
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
                    ui.label(gen_rich_text(ctx, locale.title.as_str(), Heading, None))
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
                ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                    if ui
                        .button(gen_rich_text(ctx, locale.go.as_str(), Button, None))
                        .clicked()
                    {
                        self.try_go(&mut config, locale);
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
                        self.player_bar.show(max_height, &mut config, ui);
                    },
                );

                ui.horizontal(|ui| {
                    self.config_playlist_path
                        .show_config(ui, ctx, locale, &mut config);
                    self.config_playlist_path
                        .show_hint(ui, ctx, locale, &config);
                });

                if config.playlist_path.is_none() {
                    ui.horizontal(|ui| {
                        self.config_media_type
                            .show_config(ui, ctx, locale, &mut config.media_type);
                        self.config_media_type
                            .show_hint(ui, ctx, locale, &config.media_type);
                    });

                    ui.horizontal(|ui| {
                        self.config_root_path
                            .show_config(ui, ctx, locale, &mut config);
                        self.config_root_path.show_hint(ui, ctx, locale, &config);
                    });

                    if config.media_type == MediaType::Video {
                        ui.horizontal(|ui| {
                            self.config_video_player.show_config(
                                ui,
                                ctx,
                                locale,
                                &mut config.video_player,
                            );
                            self.config_video_player.show_hint(
                                ui,
                                ctx,
                                locale,
                                &config.video_player,
                            );
                        });

                        match config.video_player {
                            #[cfg(feature = "native")]
                            VideoPlayer::Native => {}
                            _ => {
                                ui.horizontal(|ui| {
                                    self.config_video_player_path.show_config(
                                        ui,
                                        ctx,
                                        locale,
                                        &mut config.video_player_path,
                                    );
                                    self.config_video_player_path.show_hint(
                                        ui,
                                        ctx,
                                        locale,
                                        &config.video_player_path,
                                    );
                                });
                            }
                        }
                    }
                }
            });
    }
}
