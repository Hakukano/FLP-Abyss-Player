use eframe::{
    egui::{self, style::Margin, Frame, TextStyle::*},
    epaint::Color32,
};

use crate::{config::*, font::gen_rich_text, locale};

#[derive(Default)]
pub struct State {
    alert: bool,
    alert_message: Option<String>,
    go: bool,
}

impl State {
    pub fn reset(&mut self) {
        self.alert = false;
        self.alert_message.take();
        self.go = false;
    }

    pub fn should_go(&self) -> bool {
        self.go
    }

    pub fn update(&mut self, ctx: &egui::Context) {
        let mut config = get().lock().expect("Cannot get config lock");
        let locale = &locale::get().ui.config;

        if let Some(alert_message) = &self.alert_message {
            egui::Window::new("").open(&mut self.alert).show(ctx, |ui| {
                ui.label(gen_rich_text(
                    ctx,
                    alert_message,
                    Heading,
                    Some(Color32::RED),
                ));
            });
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
                        let path_set = config.root_path.is_some();
                        let other_set = match config.media_type {
                            MediaType::Image => true,
                            MediaType::Video => {
                                !config.video_player.is_unset()
                                    && config.video_player_path.is_some()
                            }
                            _ => false,
                        };
                        if path_set && other_set {
                            self.go = true;
                        } else {
                            self.alert_message.replace(locale.alert.clone());
                            self.alert = true;
                        }
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
                                gen_rich_text(ctx, locale.video_player.qtp.as_str(), Body, None),
                            );
                            ui.selectable_value(
                                &mut config.video_player,
                                VideoPlayer::Vlc,
                                gen_rich_text(ctx, locale.video_player.vlc.as_str(), Body, None),
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
            });
    }
}
