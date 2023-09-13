use eframe::egui::{
    Align, CentralPanel, Context, DragValue, Frame, Key, Layout, Margin, TextStyle::*,
    TopBottomPanel, Ui, Vec2, Window,
};
use rand::Rng;
use std::{
    path::{Path, PathBuf},
    sync::{mpsc::Sender, Arc},
};

use crate::{
    controller::{Command, CommandName, ControllerType},
    library::fonts::gen_rich_text,
    model::{
        config::{Config, MediaType},
        player::Player,
    },
    view::{
        widget::{button_icon::ButtonIcon, player_bar::PlayerBar, playlist::Playlist},
        Packet, PacketName, ViewType,
    },
    CLI,
};

mod image;
mod server;
mod video;

pub trait MediaPlayer: Send + Sync {
    fn is_loaded(&self) -> bool;
    fn is_end(&self) -> bool;
    fn reload(&mut self, path: &dyn AsRef<Path>, ctx: &Context, state: &Player);
    fn sync(&mut self, state: &Player);
    fn show_central_panel(&mut self, ui: &mut Ui, ctx: &Context, can_input: bool);
}

pub struct View {
    packet_tx: Sender<Packet>,
    command_tx: Sender<Command>,

    state: Player,
    state_buffer: Player,
    index: usize,
    index_buffer: usize,

    search: bool,
    search_str: String,
    filtered_paths: Vec<(usize, String)>,

    save: Option<PathBuf>,
    load: Option<PathBuf>,

    player_bar: PlayerBar,
    prev_icon: ButtonIcon,
    next_icon: ButtonIcon,
    playlist_icon: ButtonIcon,
    playlist: Playlist,

    show_playlist: bool,

    media_player: Box<dyn MediaPlayer>,
}

impl View {
    pub fn new(
        player: Player,
        packet_tx: Sender<Packet>,
        command_tx: Sender<Command>,
        ctx: &Context,
        #[cfg(feature = "native")] gl: Arc<glow::Context>,
    ) -> Self {
        let mut media_player: Box<dyn MediaPlayer> = match player.playlist.header.media_type {
            MediaType::Server => Box::new(server::MediaPlayer::new()),
            MediaType::Image => Box::new(image::MediaPlayer::new()),
            MediaType::Video => Box::new(video::MediaPlayer::new(
                ctx,
                #[cfg(feature = "native")]
                gl,
            )),
            _ => panic!("Unknown media type"),
        };
        media_player.reload(
            player
                .playlist
                .body
                .item_paths
                .get(0)
                .expect("Out of bound: paths"),
            ctx,
            &player,
        );
        let icon_path = Path::new(CLI.assets_path.as_str())
            .join("image")
            .join("icon");
        Self {
            packet_tx,
            command_tx,

            state: player.clone(),
            state_buffer: player.clone(),
            index: 0,
            index_buffer: 0,

            search: false,
            search_str: String::new(),
            filtered_paths: player
                .playlist
                .body
                .item_paths
                .iter()
                .enumerate()
                .map(|(i, p)| (i, p.clone()))
                .collect(),

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
            playlist: Playlist::new(ctx),

            show_playlist: false,

            media_player,
        }
    }

    fn can_input(&self) -> bool {
        !self.show_playlist
    }

    pub fn item_paths(&self) -> &[String] {
        self.state.playlist.body.item_paths.as_slice()
    }

    pub fn set_index(&mut self, index: usize, ctx: &Context) {
        self.index = index;
        self.index_buffer = index;
        self.media_player.reload(
            self.state
                .playlist
                .body
                .item_paths
                .get(index)
                .expect("Out of bound: paths"),
            ctx,
            &self.state,
        );
    }

    pub fn random(&mut self) -> usize {
        let mut rng = rand::thread_rng();
        rng.gen_range(0..self.item_paths().len())
    }

    pub fn next(&mut self) -> usize {
        if self.state.repeat {
            return self.index;
        }
        if self.state.random {
            return self.random();
        }
        if self.index == self.item_paths().len() - 1 && self.state.lop {
            0
        } else if self.index < self.item_paths().len() - 1 {
            self.index + 1
        } else {
            self.index
        }
    }

    pub fn prev(&mut self) -> usize {
        if self.state.repeat {
            return self.index;
        }
        if self.state.random {
            return self.random();
        }
        if self.index == 0 && self.state.lop {
            self.item_paths().len() - 1
        } else if self.index > 0 {
            self.index - 1
        } else {
            self.index
        }
    }
}

impl super::View for View {
    fn handle(&mut self, packet: Packet) {
        match packet.name {
            PacketName::Update => {
                let player: Player = serde_json::from_value(packet.data).unwrap();
                self.state = player;
                self.state_buffer = self.state.clone();
                self.filtered_paths = self.state.playlist.filter(self.search_str.as_str());
                self.media_player.sync(&self.state);
            }
            PacketName::Filter => {
                self.filtered_paths = serde_json::from_value(packet.data).unwrap();
            }
            _ => {}
        }
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
                    &mut self.index_buffer,
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
                                self.index_buffer = self.next();
                            }
                            ui.label(gen_rich_text(
                                ctx,
                                format!("/{}", self.item_paths().len()),
                                Body,
                                None,
                            ));
                            let mut idx = self.index;
                            if ui
                                .add(
                                    DragValue::new(&mut idx)
                                        .speed(1)
                                        .clamp_range(0..=(self.item_paths().len() - 1))
                                        .custom_formatter(|n, _| (n as usize + 1).to_string())
                                        .custom_parser(|s| {
                                            s.parse::<usize>().map(|n| (n - 1) as f64).ok()
                                        }),
                                )
                                .changed()
                            {
                                self.index_buffer = idx;
                            }
                            if self.prev_icon.show(max_size, ui).clicked() {
                                self.index_buffer = self.prev();
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
                            .get(self.index)
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
                if self.item_paths().is_empty() {
                    return;
                }
                self.media_player
                    .show_central_panel(ui, ctx, self.can_input());
            });

        if self.can_input() {
            if ctx.input(|i| i.key_pressed(Key::J)) {
                self.index_buffer = self.next();
            }
            if ctx.input(|i| i.key_pressed(Key::K)) {
                self.index_buffer = self.prev();
            }
            if ctx.input(|i| i.key_pressed(Key::R)) {
                self.index_buffer = self.random();
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
        if let Some(diff) = self.state.diff(&self.state_buffer) {
            self.command_tx
                .send(Command::new(
                    ControllerType::Player,
                    CommandName::Update,
                    diff,
                ))
                .unwrap();
        }
        if self.index != self.index_buffer {
            self.set_index(self.index_buffer, ctx);
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