use eframe::{
    egui::{self, style::Margin, Frame, Layout, TextStyle::*},
    emath::Align,
    epaint::Color32,
};

use crate::{
    config::*, font::gen_rich_text, get_cli, helper::message_dialog_error, locale, playlist,
    widget::player_bar::PlayerBar,
};

pub struct State {
    player_bar: PlayerBar,

    go: bool,

    pub playlist: Option<(playlist::Header, playlist::Body)>,
}

impl State {
    pub fn new(ctx: &egui::Context) -> Self {
        Self {
            player_bar: PlayerBar::new(ctx),

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
            MediaType::Video => {
                !config.video_player.is_unset() && config.video_player_path.is_some()
            }
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
                    if ui
                        .button(gen_rich_text(
                            ctx,
                            locale.playlist_path.label.as_str(),
                            Body,
                            None,
                        ))
                        .clicked()
                    {
                        if let Some(path) = rfd::FileDialog::new().pick_file() {
                            config.playlist_path.replace(path.display().to_string());
                        }
                    }
                    if let Some(playlist_path) = &config.playlist_path {
                        ui.label(gen_rich_text(
                            ctx,
                            format!("{}: {playlist_path}", locale.playlist_path.set.as_str()),
                            Body,
                            None,
                        ));
                    } else {
                        ui.label(gen_rich_text(
                            ctx,
                            locale.playlist_path.unset.as_str(),
                            Body,
                            Some(Color32::WHITE),
                        ));
                    }
                });

                if config.playlist_path.is_none() {
                    ui.horizontal(|ui| {
                        egui::ComboBox::from_label(gen_rich_text(
                            ctx,
                            locale.media_type.label.as_str(),
                            Body,
                            None,
                        ))
                        .selected_text(gen_rich_text(
                            ctx,
                            config.media_type.to_string(),
                            Body,
                            None,
                        ))
                        .show_ui(ui, |ui| {
                            ui.selectable_value(
                                &mut config.media_type,
                                MediaType::Unset,
                                gen_rich_text(ctx, "--", Body, None),
                            );
                            ui.selectable_value(
                                &mut config.media_type,
                                MediaType::Image,
                                gen_rich_text(ctx, locale.media_type.image.as_str(), Body, None),
                            );
                            ui.selectable_value(
                                &mut config.media_type,
                                MediaType::Video,
                                gen_rich_text(ctx, locale.media_type.video.as_str(), Body, None),
                            );
                        });
                        if config.media_type.is_unset() {
                            ui.label(gen_rich_text(
                                ctx,
                                locale.media_type.unset.as_str(),
                                Body,
                                Some(Color32::LIGHT_RED),
                            ));
                        }
                    });

                    ui.horizontal(|ui| {
                        if ui
                            .button(gen_rich_text(
                                ctx,
                                locale.root_path.label.as_str(),
                                Body,
                                None,
                            ))
                            .clicked()
                        {
                            if let Some(path) = rfd::FileDialog::new().pick_folder() {
                                config.root_path.replace(path.display().to_string());
                            }
                        }
                        if let Some(root_path) = &config.root_path {
                            ui.label(gen_rich_text(
                                ctx,
                                format!("{}: {root_path}", locale.root_path.set.as_str()),
                                Body,
                                None,
                            ));
                        } else {
                            ui.label(gen_rich_text(
                                ctx,
                                locale.root_path.unset.as_str(),
                                Body,
                                Some(Color32::LIGHT_RED),
                            ));
                        }
                    });

                    if config.media_type == MediaType::Video {
                        ui.horizontal(|ui| {
                            egui::ComboBox::from_label(gen_rich_text(
                                ctx,
                                locale.video_player.label.as_str(),
                                Body,
                                None,
                            ))
                            .selected_text(gen_rich_text(
                                ctx,
                                config.video_player.to_string(),
                                Body,
                                None,
                            ))
                            .show_ui(ui, |ui| {
                                ui.selectable_value(
                                    &mut config.video_player,
                                    VideoPlayer::Unset,
                                    gen_rich_text(ctx, "--", Body, None),
                                );
                                ui.selectable_value(
                                    &mut config.video_player,
                                    VideoPlayer::Qtp,
                                    gen_rich_text(
                                        ctx,
                                        locale.video_player.qtp.as_str(),
                                        Body,
                                        None,
                                    ),
                                );
                                ui.selectable_value(
                                    &mut config.video_player,
                                    VideoPlayer::Vlc,
                                    gen_rich_text(
                                        ctx,
                                        locale.video_player.vlc.as_str(),
                                        Body,
                                        None,
                                    ),
                                );
                            });
                            if config.video_player.is_unset() {
                                ui.label(gen_rich_text(
                                    ctx,
                                    locale.video_player.unset.as_str(),
                                    Body,
                                    Some(Color32::LIGHT_RED),
                                ));
                            }
                        });

                        ui.horizontal(|ui| {
                            if ui
                                .button(gen_rich_text(
                                    ctx,
                                    locale.video_player_path.label.as_str(),
                                    Body,
                                    None,
                                ))
                                .clicked()
                            {
                                if let Some(path) = rfd::FileDialog::new().pick_file() {
                                    config.video_player_path.replace(path.display().to_string());
                                }
                            }
                            if let Some(video_player_path) = &config.video_player_path {
                                ui.label(gen_rich_text(
                                    ctx,
                                    format!(
                                        "{}: {video_player_path}",
                                        locale.video_player_path.set.as_str()
                                    ),
                                    Body,
                                    None,
                                ));
                            } else {
                                ui.label(gen_rich_text(
                                    ctx,
                                    locale.video_player_path.unset.as_str(),
                                    Body,
                                    Some(Color32::LIGHT_RED),
                                ));
                            }
                        });
                    }
                }
            });
    }
}
