use eframe::egui::{
    Align, CentralPanel, Context, DragValue, Frame, Key, Layout, Margin, TextStyle::*,
    TopBottomPanel, Ui, Vec2, Window,
};
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    sync::{mpsc::Sender, Arc},
};

use crate::{
    models::{config::MediaType, player::Player},
    utils::{cli::CLI, fonts::gen_rich_text},
    views::widgets::{button_icon::ButtonIcon, player_bar::PlayerBar, playlist::PlaylistWidget},
};

use super::{timer::TimerSignal, ChangeLocation};

mod image;
mod server;
mod video;

enum MediaPlayer {
    Server(server::MediaPlayer),
    Image(image::MediaPlayer),
    Video(video::MediaPlayer),
}

impl MediaPlayer {
    fn new(player: &Player, ctx: &Context, gl: Arc<glow::Context>) -> Self {
        let playlist = player.playlist().expect("Playlist not found");
        match playlist.header.media_type {
            MediaType::Server => Self::Server(server::MediaPlayer::new(
                playlist.item_paths().iter().map(PathBuf::from).collect(),
            )),
            MediaType::Image => Self::Image(image::MediaPlayer::new(player, ctx)),
            MediaType::Video => Self::Video(video::MediaPlayer::new(player, ctx, gl)),
            _ => panic!("Unknown media type"),
        }
    }

    fn update(&mut self, ui: &mut Ui, ctx: &Context, can_input: bool) {
        match self {
            MediaPlayer::Server(media_player) => media_player.update(ui, ctx),
            MediaPlayer::Image(media_player) => media_player.update(ui),
            MediaPlayer::Video(media_player) => media_player.update(ui, ctx, can_input),
        }
    }

    fn is_end(&self) -> bool {
        match self {
            MediaPlayer::Server(_) => true,
            MediaPlayer::Image(_) => true,
            MediaPlayer::Video(media_player) => media_player.is_end(),
        }
    }
}

pub struct View {
    change_location_tx: Sender<ChangeLocation>,
    timer_signal_tx: Sender<TimerSignal>,
    gl: Arc<glow::Context>,

    playlist: PlaylistWidget,
    player: Player,

    player_bar: PlayerBar,
    prev_icon: ButtonIcon,
    next_icon: ButtonIcon,
    playlist_icon: ButtonIcon,

    show_playlist: bool,

    media_player: MediaPlayer,
}

impl View {
    pub fn new(
        id: &str,
        change_location_tx: Sender<ChangeLocation>,
        timer_signal_tx: Sender<TimerSignal>,
        ctx: &Context,
        gl: Arc<glow::Context>,
    ) -> Self {
        let player = Player::find(id).expect("Player not found");
        let playlist = PlaylistWidget::new(
            player.playlist().expect("Playlist not found").id.as_str(),
            ctx,
        );

        let icon_path = Path::new(CLI.assets_path.as_str())
            .join("image")
            .join("icon");

        let media_player = MediaPlayer::new(&player, ctx, gl.clone());

        Self {
            change_location_tx,
            timer_signal_tx,
            gl,

            player: player.clone(),

            player_bar: PlayerBar::new(ctx),
            prev_icon: ButtonIcon::from_rgba_image_files("prev", icon_path.join("prev.png"), ctx),
            next_icon: ButtonIcon::from_rgba_image_files("next", icon_path.join("next.png"), ctx),
            playlist_icon: ButtonIcon::from_rgba_image_files(
                "playlist",
                icon_path.join("playlist.png"),
                ctx,
            ),
            playlist,

            show_playlist: false,

            media_player,
        }
    }

    pub fn update(&mut self, ctx: &Context, need_to_tick: bool) {
        let mut need_to_reload = false;

        if self.media_player.is_end() && need_to_tick {
            self.player.next();
            need_to_reload = true;
        }

        Window::new("playlist")
            .resizable(true)
            .default_size(Vec2::new(600.0, 600.0))
            .open(&mut self.show_playlist)
            .show(ctx, |ui| {
                self.playlist.update(ui, ctx, &mut self.player.index);
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
                            self.player_bar.update(
                                &self.timer_signal_tx,
                                max_height,
                                ui,
                                &mut self.player.repeat,
                                &mut self.player.auto,
                                &mut self.player.auto_interval,
                                &mut self.player.lop,
                                &mut self.player.random,
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
                            if self.playlist_icon.update(max_size, ui).clicked() {
                                self.show_playlist = true;
                            }
                            if self.next_icon.update(max_size, ui).clicked() {
                                self.player.next();
                                need_to_reload = true;
                            }
                            ui.label(gen_rich_text(
                                ctx,
                                format!("/{}", self.playlist.playlist.item_paths().len()),
                                Body,
                                None,
                            ));
                            ui.add(
                                DragValue::new(&mut self.player.index)
                                    .speed(1)
                                    .clamp_range(
                                        0..=(self.playlist.playlist.item_paths().len() - 1),
                                    )
                                    .custom_formatter(|n, _| (n as usize + 1).to_string())
                                    .custom_parser(|s| {
                                        s.parse::<usize>().map(|n| (n - 1) as f64).ok()
                                    }),
                            );
                            if self.prev_icon.update(max_size, ui).clicked() {
                                self.player.prev();
                                need_to_reload = true;
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
                        self.playlist
                            .playlist
                            .body
                            .item_paths
                            .get(self.player.index)
                            .expect("Out of bound: paths"),
                        Body,
                        None,
                    ));
                    ui.with_layout(Layout::right_to_left(Align::TOP), |ui| {
                        if ui
                            .button(gen_rich_text(ctx, t!("ui.view.home"), Body, None))
                            .clicked()
                        {
                            self.change_location_tx
                                .send(ChangeLocation {
                                    path: vec![
                                        "configs".to_string(),
                                        self.playlist
                                            .playlist
                                            .config()
                                            .expect("Config not found")
                                            .id,
                                    ],
                                    query: HashMap::new(),
                                })
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
                if self.playlist.playlist.item_paths().is_empty() {
                    return;
                }
                self.media_player.update(ui, ctx, self.can_input());
            });

        if self.can_input() {
            if ctx.input(|i| i.key_pressed(Key::ArrowRight)) {
                self.player.next();
                need_to_reload = true;
            }
            if ctx.input(|i| i.key_pressed(Key::ArrowLeft)) {
                self.player.prev();
                need_to_reload = true;
            }
            if ctx.input(|i| i.key_pressed(Key::R)) {
                self.player.random_next();
                need_to_reload = true;
            }
            if ctx.input(|i| i.key_pressed(Key::Num1)) {
                self.player.repeat = !self.player.repeat;
            }
            if ctx.input(|i| i.key_pressed(Key::Num2)) {
                self.player.auto = !self.player.auto;
            }
            if ctx.input(|i| i.key_pressed(Key::Num3)) {
                self.player.lop = !self.player.lop;
            }
            if ctx.input(|i| i.key_pressed(Key::Num4)) {
                self.player.random = !self.player.random;
            }
        }

        // Cleanup Phase
        self.player.save();
        if need_to_reload {
            self.media_player = MediaPlayer::new(&self.player, ctx, self.gl.clone());
        }
    }

    fn can_input(&self) -> bool {
        !self.show_playlist
    }
}
