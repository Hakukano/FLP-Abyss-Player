use std::sync::Mutex;

use eframe::{
    egui::{self, style::Margin, Frame, RichText},
    epaint::{Color32, FontId},
};
use once_cell::sync::OnceCell;

use crate::{locale, sized_text};

#[derive(Clone, PartialEq, Eq)]
pub enum MediaType {
    Unset,
    Image,
    Video,
}

impl MediaType {
    pub fn is_unset(&self) -> bool {
        matches!(self, Self::Unset)
    }
}

impl Default for MediaType {
    fn default() -> Self {
        Self::Unset
    }
}

impl ToString for MediaType {
    fn to_string(&self) -> String {
        let media_type = &locale::get().ui.config.media_type;
        match self {
            Self::Unset => "--".to_string(),
            Self::Image => media_type.image.clone(),
            Self::Video => media_type.video.clone(),
        }
    }
}

#[derive(Default)]
pub struct State {
    alert: bool,
    alert_message: Option<String>,
    go: bool,

    pub media_type: MediaType,
    pub root_path: Option<String>,
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
        let locale = &locale::get().ui.config;

        if let Some(alert_message) = &self.alert_message {
            egui::Window::new("").open(&mut self.alert).show(ctx, |ui| {
                ui.label(sized_text!(alert_message, 32.0, Color32::RED));
            });
        }

        egui::TopBottomPanel::top("title")
            .frame(Frame::none().inner_margin(Margin {
                top: 10.0,
                bottom: 10.0,
                ..Default::default()
            }))
            .show(ctx, |ui| {
                ui.centered_and_justified(|ui| ui.label(sized_text!(locale.title.as_str(), 32.0)));
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
                    if ui.button(sized_text!(locale.go.as_str(), 16.0)).clicked() {
                        if !(self.media_type.is_unset() || self.root_path.is_none()) {
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
                    egui::ComboBox::from_label(sized_text!(locale.media_type.label.as_str(), 16.0))
                        .selected_text(sized_text!(self.media_type.to_string(), 16.0))
                        .show_ui(ui, |ui| {
                            ui.selectable_value(
                                &mut self.media_type,
                                MediaType::Unset,
                                sized_text!("--", 16.0),
                            );
                            ui.selectable_value(
                                &mut self.media_type,
                                MediaType::Image,
                                sized_text!(locale.media_type.image.as_str(), 16.0),
                            );
                            ui.selectable_value(
                                &mut self.media_type,
                                MediaType::Video,
                                sized_text!(locale.media_type.video.as_str(), 16.0),
                            );
                        });
                    if self.media_type.is_unset() {
                        ui.label(sized_text!(
                            locale.media_type.unset.as_str(),
                            16.0,
                            Color32::LIGHT_RED
                        ));
                    }
                });

                ui.horizontal(|ui| {
                    if ui
                        .button(sized_text!(locale.root_path.label.as_str(), 16.0))
                        .clicked()
                    {
                        if let Some(path) = rfd::FileDialog::new().pick_folder() {
                            self.root_path.replace(path.display().to_string());
                        }
                    }
                    if let Some(root_path) = &self.root_path {
                        ui.label(sized_text!(
                            format!("{}: {root_path}", locale.root_path.set.as_str()),
                            16.0
                        ));
                    } else {
                        ui.label(sized_text!(
                            locale.root_path.unset.as_str(),
                            16.0,
                            Color32::LIGHT_RED
                        ));
                    }
                });
            });
    }
}

pub fn get() -> &'static Mutex<State> {
    static CONFIG: OnceCell<Mutex<State>> = OnceCell::new();
    CONFIG.get_or_init(|| Mutex::new(State::default()))
}
