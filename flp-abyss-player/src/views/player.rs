use eframe::egui::{
    Align, CentralPanel, Context, DragValue, Frame, Key, Layout, Margin, TextStyle::*,
    TopBottomPanel, Vec2, Window,
};
use serde_json::Value;
use std::{
    path::{Path, PathBuf},
    sync::{mpsc::Sender, Arc},
};

use crate::{
    models::{
        config::{Config, MediaType},
        player::Player,
    },
    utils::{cli::CLI, differ::Differ, fonts::gen_rich_text},
    views::widgets::{button_icon::ButtonIcon, player_bar::PlayerBar, playlist::PlaylistWidget},
};

mod image;
mod server;
mod video;

enum MediaPlayer {
    Server(server::MediaPlayer),
    Image(image::MediaPlayer),
    Video(image::MediaPlayer),
}

pub struct View {
    playlist: PlaylistWidget,
    player: Player,

    search: bool,
    search_str: String,
    filtered_paths: Vec<(usize, String)>,

    save: Option<PathBuf>,
    load: Option<PathBuf>,

    player_bar: PlayerBar,
    prev_icon: ButtonIcon,
    next_icon: ButtonIcon,
    playlist_icon: ButtonIcon,

    show_playlist: bool,

    media_player: MediaPlayer,
}

impl View {
    pub fn new(id: &str, ctx: &Context, gl: Arc<glow::Context>) -> Self {
        let player = Player::find(id).expect("Player not found");
        let playlist = player.playlist().expect("Playlist not found");

        let filtered_paths = playlist
            .item_paths()
            .iter()
            .enumerate()
            .map(|(i, p)| (i, p.clone()))
            .collect();

        let icon_path = Path::new(CLI.assets_path.as_str())
            .join("image")
            .join("icon");

        let mut media_player = match playlist.header.media_type {
            MediaType::Server => MediaPlayer::Server(server::MediaPlayer::new()),
            MediaType::Image => MediaPlayer::Image(image::MediaPlayer::new()),
            MediaType::Video => MediaPlayer::Video(video::MediaPlayer::new(ctx, gl)),
            _ => panic!("Unknown media type"),
        };

        Self {
            player: player.clone(),

            search: false,
            search_str: String::new(),
            filtered_paths,

            save: None,
            load: None,

            player_bar: PlayerBar::new(ctx),
            prev_icon: ButtonIcon::from_rgba_image_files("prev", icon_path.join("prev.png"), ctx),
            next_icon: ButtonIcon::from_rgba_image_files("next", icon_path.join("next.png"), ctx),
            playlist_icon: ButtonIcon::from_rgba_image_files(
                "playlist",
                icon_path.join("playlist.png"),
                ctx,
            ),
            playlist: PlaylistWidget::new(playlist, ctx),

            show_playlist: false,

            media_player,
        }
    }

    fn can_input(&self) -> bool {
        !self.show_playlist
    }

    pub fn reload_media_player(&mut self, ctx: &Context) {
        self.media_player.reload(
            self.state
                .playlist
                .body
                .item_paths
                .get(self.state_buffer.index)
                .expect("Out of bound: paths"),
            ctx,
            &self.state_buffer,
        );
    }

    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        Window::new("playlist")
            .resizable(true)
            .default_size(Vec2::new(600.0, 600.0))
            .open(&mut self.show_playlist)
            .show(ctx, |ui| {
                self.playlist.show(
                    ui,
                    ctx,
                    &mut self.state_buffer,
                    &mut self.search,
                    &mut self.search_str,
                    &self.filtered_paths,
                    &mut self.save,
                    &mut self.load,
                );
            });

        TopBottomPanel::top("title")
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
                            self.player_bar.show(
                                max_height,
                                ui,
                                &mut self.state_buffer.repeat,
                                &mut self.state_buffer.auto,
                                &mut self.state_buffer.auto_interval,
                                &mut self.state_buffer.lop,
                                &mut self.state_buffer.random,
                            );
                        },
                    );
                    ui.with_layout(
                        Layout::right_to_left(Align::TOP).with_cross_justify(true),
                        |ui| {
                            ui.add_space(10.0);
                            ui.spacing_mut().item_spacing = Vec2::new(8.0, 8.0);
                            ui.style_mut().drag_value_text_style = Body;
                            let max_size = Vec2::new(20.0, 20.0);
                            if self.playlist_icon.show(max_size, ui).clicked() {
                                self.show_playlist = true;
                            }
                            if self.next_icon.show(max_size, ui).clicked() {
                                self.state_buffer.next();
                            }
                            ui.label(gen_rich_text(
                                ctx,
                                format!("/{}", self.state.item_paths().len()),
                                Body,
                                None,
                            ));
                            ui.add(
                                DragValue::new(&mut self.state_buffer.index)
                                    .speed(1)
                                    .clamp_range(0..=(self.state.item_paths().len() - 1))
                                    .custom_formatter(|n, _| (n as usize + 1).to_string())
                                    .custom_parser(|s| {
                                        s.parse::<usize>().map(|n| (n - 1) as f64).ok()
                                    }),
                            );
                            if self.prev_icon.show(max_size, ui).clicked() {
                                self.state_buffer.prev();
                            }
                        },
                    );
                });
            });

        TopBottomPanel::bottom("home")
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
                        self.state
                            .playlist
                            .body
                            .item_paths
                            .get(self.state.index)
                            .expect("Out of bound: paths"),
                        Body,
                        None,
                    ));
                    ui.with_layout(Layout::right_to_left(Align::TOP), |ui| {
                        if ui
                            .button(gen_rich_text(ctx, t!("ui.view.home"), Body, None))
                            .clicked()
                        {
                            self.packet_tx
                                .send(Packet::new(
                                    PacketName::ChangeView(ViewType::Config),
                                    serde_json::to_value(Config::all()).unwrap(),
                                ))
                                .unwrap();
                        }
                    });
                })
            });

        CentralPanel::default()
            .frame(Frame::none().inner_margin(Margin {
                top: 10.0,
                bottom: 10.0,
                right: 10.0,
                left: 10.0,
            }))
            .show(ctx, |ui| {
                if self.state.item_paths().is_empty() {
                    return;
                }
                self.media_player
                    .show_central_panel(ui, ctx, self.can_input());
            });

        if self.can_input() {
            if ctx.input(|i| i.key_pressed(Key::J)) {
                self.state_buffer.next();
            }
            if ctx.input(|i| i.key_pressed(Key::K)) {
                self.state_buffer.prev();
            }
            if ctx.input(|i| i.key_pressed(Key::R)) {
                self.state_buffer.random_next();
            }
            if ctx.input(|i| i.key_pressed(Key::Num1)) {
                self.state_buffer.repeat = !self.state_buffer.repeat;
            }
            if ctx.input(|i| i.key_pressed(Key::Num2)) {
                self.state_buffer.auto = !self.state_buffer.auto;
            }
            if ctx.input(|i| i.key_pressed(Key::Num3)) {
                self.state_buffer.lop = !self.state_buffer.lop;
            }
            if ctx.input(|i| i.key_pressed(Key::Num4)) {
                self.state_buffer.random = !self.state_buffer.random;
            }
        }

        // Cleanup Phase
        if self.state.auto != self.state_buffer.auto {
            if self.state_buffer.auto {
                self.signal_tx
                    .send(Signal::new(
                        SignalName::Start,
                        serde_json::to_value(self.state_buffer.auto_interval).unwrap(),
                    ))
                    .unwrap();
            } else {
                self.signal_tx
                    .send(Signal::new(SignalName::Stop, Value::Null))
                    .unwrap();
            }
        }
        if self.state.auto_interval != self.state_buffer.auto_interval {
            self.signal_tx
                .send(Signal::new(
                    SignalName::Update,
                    serde_json::to_value(self.state_buffer.auto_interval).unwrap(),
                ))
                .unwrap();
        }
        if self.state.index != self.state_buffer.index {
            self.reload_media_player(ctx);
        }
        if let Some(diff) = self.state.diff(&self.state_buffer) {
            self.command_tx
                .send(Command::new(
                    ControllerType::Player,
                    CommandName::Update,
                    diff,
                ))
                .unwrap();
        }
        if self.search {
            self.command_tx
                .send(Command::new(
                    ControllerType::Player,
                    CommandName::Search,
                    serde_json::to_value(self.search_str.as_str()).unwrap(),
                ))
                .unwrap();
            self.search = false;
        }
        if let Some(path) = self.save.take() {
            self.command_tx
                .send(Command::new(
                    ControllerType::Player,
                    CommandName::Save,
                    serde_json::to_value(path.to_str().unwrap()).unwrap(),
                ))
                .unwrap();
        }
        if let Some(path) = self.load.take() {
            self.command_tx
                .send(Command::new(
                    ControllerType::Player,
                    CommandName::Load,
                    serde_json::to_value(path.to_str().unwrap()).unwrap(),
                ))
                .unwrap();
        }
    }
}
