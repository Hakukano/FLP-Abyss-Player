use eframe::egui::{
    Align, CentralPanel, Context, DragValue, Frame, Key, Layout, Margin, TextStyle::*,
    TopBottomPanel, Vec2, Window,
};
use std::{
    collections::HashMap,
    path::Path,
    sync::{mpsc::Sender, Arc},
};

use crate::{
    models::{config::MediaType, player::Player},
    utils::{cli::CLI, fonts::gen_rich_text},
    views::widgets::{button_icon::ButtonIcon, player_bar::PlayerBar, playlist::PlaylistWidget},
};

use super::ChangeLocation;

mod image;
mod server;
mod video;

enum MediaPlayer {
    Server(server::MediaPlayer),
    Image(image::MediaPlayer),
    Video(image::MediaPlayer),
}

pub struct View {
    change_location_tx: Sender<Vec<String>>,

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
        change_location_tx: Sender<Vec<String>>,
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

        let mut media_player = match playlist.header.media_type {
            MediaType::Server => MediaPlayer::Server(server::MediaPlayer::new()),
            MediaType::Image => MediaPlayer::Image(image::MediaPlayer::new()),
            MediaType::Video => MediaPlayer::Video(video::MediaPlayer::new(ctx, gl)),
            _ => panic!("Unknown media type"),
        };

        Self {
            change_location_tx,

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

    pub fn update(&mut self, ctx: &Context) {
        Window::new("playlist")
            .resizable(true)
            .default_size(Vec2::new(600.0, 600.0))
            .open(&mut self.show_playlist)
            .show(ctx, |ui| {
                self.playlist.show(ui, ctx, &mut self.player.index);
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
                            if self.playlist_icon.show(max_size, ui).clicked() {
                                self.show_playlist = true;
                            }
                            if self.next_icon.show(max_size, ui).clicked() {
                                self.state_buffer.next();
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
                            if self.prev_icon.show(max_size, ui).clicked() {
                                self.player.prev();
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
        self.player.save();
    }
}
