mod image;
mod video;

use std::{
    ffi::OsStr,
    path::{Path, PathBuf},
    sync::{Arc, RwLock},
};

use chrono::{DateTime, Utc};
use eframe::{
    egui::{self, style::Margin, Frame, Key, Layout, TextStyle::*},
    emath::Align,
    epaint::Vec2,
};
use rand::Rng;
use tokio::runtime::{self, Runtime};
use walkdir::DirEntry;

use crate::{
    config::{self, MediaType},
    font::gen_rich_text,
    get_cli, locale, playlist,
    widget::{
        button_icon::ButtonIcon,
        player_bar::PlayerBar,
        playlist::{Playlist, PlaylistState},
    },
};

const RUNTIME_THREADS: usize = 2;
const RUNTIME_THREAD_NAME: &str = "view";
const RUNTIME_THREAD_STACK_SIZE: usize = 3 * 1024 * 1024;
const STATUS_SYNC_INTERVAL_MS: u64 = 300;

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

trait MediaPlayer: Send + Sync {
    fn is_loaded(&self) -> bool;
    fn is_end(&self) -> bool;
    fn support_extensions(&self) -> &[String];
    fn reload(&mut self, path: &dyn AsRef<Path>, ctx: &egui::Context);
    fn show_central_panel(&mut self, ui: &mut egui::Ui, ctx: &egui::Context, can_input: bool);
}

pub struct State {
    player_bar: PlayerBar,
    prev_icon: ButtonIcon,
    next_icon: ButtonIcon,
    playlist_icon: ButtonIcon,
    playlist: Playlist,

    show_playlist: bool,
    home: bool,

    paths: Vec<PathBuf>,
    index: usize,
    next: bool,

    media_player: Box<dyn MediaPlayer>,

    playlist_state: PlaylistState,
}

impl State {
    pub fn new(ctx: &egui::Context, playlist_body: Option<&playlist::Body>) -> Self {
        let media_type = {
            config::get()
                .read()
                .expect("Cannot get config lock")
                .media_type
        };
        let mut media_player: Box<dyn MediaPlayer> = match media_type {
            MediaType::Image => Box::new(image::MediaPlayer::new()),
            MediaType::Video => Box::new(video::MediaPlayer::new()),
            _ => panic!("Unknown media type"),
        };
        let mut paths = Vec::new();
        if let Some(playlist_body) = playlist_body {
            playlist_body.write_paths(&mut paths);
        } else {
            let root_path = {
                config::get()
                    .read()
                    .expect("Cannot get config lock")
                    .root_path
                    .clone()
                    .expect("There must be a root path at this point")
            };
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
        let icon_path = Path::new(get_cli().assets_path.as_str())
            .join("image")
            .join("icon");
        Self {
            player_bar: PlayerBar::new(ctx),
            prev_icon: ButtonIcon::from_rgba_image_files("prev", icon_path.join("prev.png"), ctx),
            next_icon: ButtonIcon::from_rgba_image_files("next", icon_path.join("next.png"), ctx),
            playlist_icon: ButtonIcon::from_rgba_image_files(
                "playlist",
                icon_path.join("playlist.png"),
                ctx,
            ),
            playlist: Playlist::new(ctx),
            show_playlist: false,
            home: false,
            paths: paths.clone(),
            index: 0,
            next: false,
            media_player,
            playlist_state: PlaylistState::new(paths, 0),
        }
    }

    pub fn should_home(&self) -> bool {
        self.home
    }

    fn can_input(&self) -> bool {
        !self.show_playlist
    }

    pub fn set_index(&mut self, index: usize, ctx: &egui::Context) {
        self.index = index;
        self.playlist_state.set_index(index);
        self.media_player
            .reload(self.paths.get(index).expect("Out of bound: paths"), ctx)
    }

    pub fn random(&mut self, ctx: &egui::Context) {
        let mut rng = rand::thread_rng();
        self.set_index(rng.gen_range(0..self.paths.len()), ctx);
    }

    pub fn next(&mut self, ctx: &egui::Context) {
        let (repeat, random, lop) = {
            let config = config::get().read().expect("Cannot get config lock");
            (config.repeat, config.random, config.lop)
        };
        if repeat {
            self.set_index(self.index, ctx);
            return;
        }
        if random {
            self.random(ctx);
            return;
        }
        if self.index == self.paths.len() - 1 && lop {
            self.set_index(0, ctx);
        } else if self.index < self.paths.len() - 1 {
            self.set_index(self.index + 1, ctx);
        }
    }

    pub fn prev(&mut self, ctx: &egui::Context) {
        let (repeat, random, lop) = {
            let config = config::get().read().expect("Cannot get config lock");
            (config.repeat, config.random, config.lop)
        };
        if repeat {
            self.set_index(self.index, ctx);
            return;
        }
        if random {
            self.random(ctx);
            return;
        }
        if self.index == 0 && lop {
            self.set_index(self.paths.len() - 1, ctx);
        } else if self.index > 0 {
            self.set_index(self.index - 1, ctx);
        }
    }

    pub fn update(&mut self, ctx: &egui::Context) {
        if let Some(paths) = self
            .playlist_state
            .consume_paths_change()
            .map(|ps| ps.to_vec())
        {
            self.paths = paths;
        }
        if let Some(index) = self.playlist_state.consume_index_change() {
            self.set_index(index, ctx);
        }

        if self.next {
            self.next = false;
            self.next(ctx);
        }

        let locale = &locale::get().ui.view;

        egui::Window::new("playlist")
            .resizable(true)
            .default_size(Vec2::new(600.0, 600.0))
            .open(&mut self.show_playlist)
            .show(ctx, |ui| {
                self.playlist.show(
                    &mut self.playlist_state,
                    &mut config::get().write().expect("Cannot get config lock"),
                    ui,
                    ctx,
                )
            });

        egui::TopBottomPanel::top("title")
            .frame(Frame::none().inner_margin(Margin {
                top: 10.0,
                bottom: 10.0,
                ..Default::default()
            }))
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.add_space(10.0);
                    ui.with_layout(
                        Layout::left_to_right(Align::TOP).with_cross_justify(true),
                        |ui| {
                            let max_height = 20.0;
                            ui.set_height(max_height);
                            ui.style_mut().drag_value_text_style = Body;
                            {
                                let mut config =
                                    config::get().write().expect("Cannot get config lock");
                                self.player_bar.show(max_height, &mut config, ui);
                            }
                        },
                    );
                    ui.with_layout(
                        egui::Layout::right_to_left(egui::Align::TOP).with_cross_justify(true),
                        |ui| {
                            ui.add_space(10.0);
                            ui.spacing_mut().item_spacing = Vec2::new(8.0, 8.0);
                            ui.style_mut().drag_value_text_style = Body;
                            let max_size = Vec2::new(20.0, 20.0);
                            if self.playlist_icon.show(max_size, ui).clicked() {
                                self.show_playlist = true;
                            }
                            if self.next_icon.show(max_size, ui).clicked() {
                                self.next(ctx);
                            }
                            ui.label(gen_rich_text(
                                ctx,
                                format!("/{}", self.paths.len()),
                                Body,
                                None,
                            ));
                            let mut idx = self.index;
                            if ui
                                .add(
                                    egui::DragValue::new(&mut idx)
                                        .speed(1)
                                        .clamp_range(0..=(self.paths.len() - 1))
                                        .custom_formatter(|n, _| (n as usize + 1).to_string())
                                        .custom_parser(|s| {
                                            s.parse::<usize>().map(|n| (n - 1) as f64).ok()
                                        }),
                                )
                                .changed()
                            {
                                self.set_index(idx, ctx);
                            }
                            if self.prev_icon.show(max_size, ui).clicked() {
                                self.prev(ctx);
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
                self.media_player
                    .show_central_panel(ui, ctx, self.can_input());
            });

        if self.can_input() {
            if ctx.input(|i| i.key_pressed(Key::J)) {
                self.next(ctx);
            }
            if ctx.input(|i| i.key_pressed(Key::K)) {
                self.prev(ctx);
            }
            if ctx.input(|i| i.key_pressed(Key::R)) {
                self.random(ctx);
            }
            if ctx.input(|i| i.key_pressed(Key::Num1)) {
                let mut config = config::get().write().expect("Cannot get config lock");
                config.repeat = !config.repeat
            }
            if ctx.input(|i| i.key_pressed(Key::Num2)) {
                let mut config = config::get().write().expect("Cannot get config lock");
                config.auto = !config.auto
            }
            if ctx.input(|i| i.key_pressed(Key::Num3)) {
                let mut config = config::get().write().expect("Cannot get config lock");
                config.lop = !config.lop
            }
            if ctx.input(|i| i.key_pressed(Key::Num4)) {
                let mut config = config::get().write().expect("Cannot get config lock");
                config.random = !config.random
            }
        }
    }
}

pub struct TimedState {
    pub state: Arc<RwLock<State>>,
    runtime: Runtime,
    end_time: Arc<RwLock<Option<DateTime<Utc>>>>,
}

impl TimedState {
    pub fn new(state: State, ctx: egui::Context) -> Self {
        let runtime = runtime::Builder::new_multi_thread()
            .worker_threads(RUNTIME_THREADS)
            .thread_name(RUNTIME_THREAD_NAME)
            .thread_stack_size(RUNTIME_THREAD_STACK_SIZE)
            .enable_all()
            .build()
            .expect("Cannot build tokio runtime");
        let timed_state = Self {
            state: Arc::new(RwLock::new(state)),
            runtime,
            end_time: Arc::new(RwLock::new(None)),
        };

        let state = timed_state.state.clone();
        let end_time = timed_state.end_time.clone();
        timed_state.runtime.spawn(async move {
            let mut timer =
                tokio::time::interval(std::time::Duration::from_millis(STATUS_SYNC_INTERVAL_MS));
            loop {
                timer.tick().await;
                let (auto, auto_interval) = {
                    let config = config::get().read().expect("Cannot get config lock");
                    (config.auto, config.auto_interval)
                };
                if !auto {
                    end_time.write().expect("Cannot get end time lock").take();
                    continue;
                }
                if state
                    .read()
                    .expect("Cannot get view state lock")
                    .media_player
                    .is_end()
                {
                    let duration_after_end = Utc::now()
                        - *end_time
                            .write()
                            .expect("Cannot get end time lock")
                            .get_or_insert_with(Utc::now);
                    let is_timed_out = duration_after_end.num_seconds() >= auto_interval as i64;
                    if is_timed_out {
                        end_time.write().expect("Cannot get end time lock").take();
                        state.write().expect("Cannot get view state lock").next = true;
                        ctx.request_repaint();
                    }
                } else {
                    end_time.write().expect("Cannot get end time lock").take();
                }
            }
        });

        timed_state
    }
}
