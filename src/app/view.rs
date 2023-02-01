use std::{
    ffi::OsStr,
    path::{Path, PathBuf},
};

use eframe::{
    egui::{self, style::Margin, Frame, Key, RichText},
    epaint::{Color32, FontId, Vec2},
};
use rand::Rng;
use walkdir::DirEntry;

use crate::{locale, sized_text};

use super::config::{self, MediaType};

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

pub struct State {
    home: bool,

    media_type: MediaType,
    paths: Vec<PathBuf>,
    index: usize,

    image: Option<image::RgbaImage>,
    texture: Option<egui::TextureHandle>,
}

impl State {
    pub fn new() -> Self {
        let mut paths = Vec::new();
        let (media_type, root_path) = {
            let config = config::get().lock().expect("Cannot get config lock");
            (config.media_type.clone(), config.root_path.clone())
        };
        if let Some(root_path) = root_path {
            for entry in walkdir::WalkDir::new(root_path)
                .into_iter()
                .filter_entry(|e| !is_hidden(e))
                .filter_map(|e| e.ok())
            {
                if match media_type {
                    MediaType::Image => entry
                        .path()
                        .has_extension(&["bmp", "gif", "jpeg", "jpg", "png"]),
                    MediaType::Video => entry.path().has_extension(&["avi", "mov", "mp4", "wmv"]),
                    _ => false,
                } {
                    paths.push(entry.path().to_path_buf());
                }
            }
        }
        Self {
            home: false,
            media_type,
            paths,
            index: 0,
            image: None,
            texture: None,
        }
    }

    pub fn reset(&mut self) {
        self.home = false;
        self.paths.clear();
        self.index = 0;
        self.image.take();
        self.texture.take();
    }

    pub fn should_home(&self) -> bool {
        self.home
    }

    pub fn set_index(&mut self, index: usize, ctx: &egui::Context) {
        self.index = index;
        match self.media_type {
            MediaType::Image => {
                self.image.replace(
                    image::io::Reader::open(
                        self.paths
                            .get(self.index)
                            .expect("Out of bound: image paths"),
                    )
                    .expect("Cannot open image file")
                    .decode()
                    .expect("Cannot decode image file")
                    .to_rgba8(),
                );
                let image = self
                    .image
                    .as_ref()
                    .expect("Impossible: self.image was just replaced");
                self.texture.replace(ctx.load_texture(
                    "current_image",
                    egui::ColorImage::from_rgba_unmultiplied(
                        [image.width() as _, image.height() as _],
                        image.as_flat_samples().as_slice(),
                    ),
                    Default::default(),
                ));
            }
            MediaType::Video => {}
            _ => {}
        }
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
                ui.centered_and_justified(|ui| {
                    ui.label(sized_text!(
                        format!(
                            "{} {}/{}",
                            locale.title.as_str(),
                            self.index + 1,
                            self.paths.len()
                        ),
                        32.0
                    ))
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
                    ui.label(sized_text!(
                        self.paths
                            .get(self.index)
                            .expect("Out of bound: paths")
                            .to_str()
                            .expect("Invalid path string"),
                        16.0
                    ));
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                        if ui.button(sized_text!(locale.home.as_str(), 16.0)).clicked() {
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

                match self.media_type {
                    MediaType::Image => {
                        if self.texture.is_none() {
                            self.set_index(0, ctx);
                        }
                        let texture = self
                            .texture
                            .as_ref()
                            .expect("Impossible: self.texture was just set");
                        let texture_size = texture.size_vec2();
                        let ui_size = ui.available_size();
                        let scale_x = ui_size.x / texture_size.x;
                        let scale_y = ui_size.y / texture_size.y;
                        let scale = scale_x.min(scale_y);
                        ui.centered_and_justified(|ui| {
                            ui.image(
                                texture,
                                Vec2::new(texture_size.x * scale, texture_size.y * scale),
                            );
                        });
                    }
                    MediaType::Video => {}
                    _ => {}
                }
            });

        if ctx.input().key_pressed(Key::ArrowRight) {
            if self.index == self.paths.len() - 1 {
                self.set_index(0, ctx);
            } else {
                self.set_index(self.index + 1, ctx);
            }
        }
        if ctx.input().key_pressed(Key::ArrowLeft) {
            if self.index == 0 {
                self.set_index(self.paths.len() - 1, ctx);
            } else {
                self.set_index(self.index - 1, ctx);
            }
        }
        if ctx.input().key_pressed(Key::R) {
            let mut rng = rand::thread_rng();
            self.set_index(rng.gen_range(0..self.paths.len()), ctx);
        }
    }
}
