mod image;
mod video;

use std::{
    ffi::OsStr,
    path::{Path, PathBuf},
};

use eframe::egui::{self, style::Margin, Frame, Key, TextStyle::*};
use rand::Rng;
use walkdir::DirEntry;

use crate::{
    config::{self, MediaType},
    font::gen_rich_text,
    locale,
};

pub trait FileExtension {
    fn has_extension<S: AsRef<str>>(&self, extensions: &[S]) -> bool;
}

impl<P: AsRef<Path>> FileExtension for P {
    fn has_extension<S: AsRef<str>>(&self, extensions: &[S]) -> bool {
        if let Some(extension) = self.as_ref().extension().and_then(OsStr::to_str) {
            return extensions
                .iter()
                .any(|x| x.as_ref().eq_ignore_ascii_case(extension));
        }

        false
    }
}

fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with('.'))
        .unwrap_or(false)
}

trait MediaPlayer {
    fn loaded(&self) -> bool;
    fn support_extensions(&self) -> &[String];
    fn reload(&mut self, path: &dyn AsRef<Path>, ctx: &egui::Context);
    fn show_central_panel(&mut self, ui: &mut egui::Ui, ctx: &egui::Context);
}

pub struct State {
    home: bool,

    paths: Vec<PathBuf>,
    index: usize,

    media_player: Box<dyn MediaPlayer>,
}

impl State {
    pub fn new(ctx: &egui::Context) -> Self {
        let mut paths = Vec::new();
        let (media_type, root_path) = {
            let config = &config::get().lock().expect("Cannot get config lock");
            (config.media_type.clone(), config.root_path.clone())
        };
        let mut media_player: Box<dyn MediaPlayer> = match media_type {
            MediaType::Image => Box::new(image::MediaPlayer::new()),
            MediaType::Video => Box::new(video::MediaPlayer::new()),
            _ => panic!("Unknown media type"),
        };
        if let Some(root_path) = root_path {
            for entry in walkdir::WalkDir::new(root_path)
                .into_iter()
                .filter_entry(|e| !is_hidden(e))
                .filter_map(|e| e.ok())
            {
                if entry
                    .path()
                    .has_extension(media_player.support_extensions())
                {
                    paths.push(entry.path().to_path_buf());
                }
            }
        }
        media_player.reload(paths.get(0).expect("Empty paths"), ctx);
        Self {
            home: false,
            paths,
            index: 0,
            media_player,
        }
    }

    pub fn reset(&mut self) {
        self.home = false;
        self.paths.clear();
        self.index = 0;
    }

    pub fn should_home(&self) -> bool {
        self.home
    }

    pub fn set_index(&mut self, index: usize, ctx: &egui::Context) {
        self.index = index;
        self.media_player
            .reload(self.paths.get(index).expect("Out of bound: paths"), ctx)
    }

    pub fn update(&mut self, ctx: &egui::Context) {
        let locale = &locale::get().ui.view;

        egui::TopBottomPanel::top("title")
            .frame(Frame::none().inner_margin(Margin {
                top: 10.0,
                bottom: 10.0,
                ..Default::default()
            }))
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.add_space(10.0);
                    ui.label(gen_rich_text(ctx, locale.title.as_str(), Heading, None));
                    ui.with_layout(
                        egui::Layout::right_to_left(egui::Align::TOP).with_cross_justify(true),
                        |ui| {
                            ui.add_space(10.0);
                            ui.style_mut().drag_value_text_style = Heading;
                            ui.label(gen_rich_text(
                                ctx,
                                format!("/{}", self.paths.len()),
                                Heading,
                                None,
                            ));
                            let response = ui.add(
                                egui::DragValue::new(&mut self.index)
                                    .speed(1)
                                    .clamp_range(0..=(self.paths.len() - 1))
                                    .custom_formatter(|n, _| (n as usize + 1).to_string())
                                    .custom_parser(|s| {
                                        s.parse::<usize>().map(|n| (n - 1) as f64).ok()
                                    }),
                            );
                            if response.changed() {
                                self.set_index(self.index, ctx);
                            }
                        },
                    );
                });
            });

        egui::TopBottomPanel::bottom("home")
            .frame(Frame::none().inner_margin(Margin {
                top: 10.0,
                bottom: 10.0,
                right: 10.0,
                ..Default::default()
            }))
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.add_space(10.0);
                    ui.label(gen_rich_text(
                        ctx,
                        self.paths
                            .get(self.index)
                            .expect("Out of bound: paths")
                            .to_str()
                            .expect("Invalid path string"),
                        Body,
                        None,
                    ));
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                        if ui
                            .button(gen_rich_text(ctx, locale.home.as_str(), Body, None))
                            .clicked()
                        {
                            self.home = true;
                        }
                    });
                })
            });

        egui::CentralPanel::default()
            .frame(Frame::none().inner_margin(Margin {
                top: 10.0,
                bottom: 10.0,
                right: 10.0,
                left: 10.0,
            }))
            .show(ctx, |ui| {
                if self.paths.is_empty() {
                    return;
                }
                self.media_player.show_central_panel(ui, ctx);
            });

        if ctx.input(|i| i.key_pressed(Key::ArrowRight)) {
            if self.index == self.paths.len() - 1 {
                self.set_index(0, ctx);
            } else {
                self.set_index(self.index + 1, ctx);
            }
        }
        if ctx.input(|i| i.key_pressed(Key::ArrowLeft)) {
            if self.index == 0 {
                self.set_index(self.paths.len() - 1, ctx);
            } else {
                self.set_index(self.index - 1, ctx);
            }
        }
        if ctx.input(|i| i.key_pressed(Key::R)) {
            let mut rng = rand::thread_rng();
            self.set_index(rng.gen_range(0..self.paths.len()), ctx);
        }
    }
}
