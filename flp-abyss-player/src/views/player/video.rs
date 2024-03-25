use anyhow::Result;
use eframe::{
    egui::{self, Context, Key, Layout, TextStyle, Ui},
    emath::Align,
    epaint::Vec2,
};
use std::{collections::VecDeque, path::Path, sync::Arc};

use crate::{
    models::{config, player::Player},
    utils::{cli::CLI, fonts::gen_rich_text, helper::seconds_to_h_m_s},
    views::widgets::button_icon::ButtonIcon,
};

mod native;
mod vlc;

const CONTROLLER_HEIGHT: f32 = 20.0;

enum VideoPlayer {
    Native(native::VideoPlayer),
    Vlc(vlc::VideoPlayer),
}

impl VideoPlayer {
    fn new(player: &Player, ctx: &Context, gl: Arc<glow::Context>) -> Self {
        let playlist = player.playlist().expect("Playlist not found");
        match playlist.header.video_player {
            config::VideoPlayer::Native => Self::Native(native::VideoPlayer::new(
                player.current_path(),
                gl,
                ctx.clone(),
            )),
            config::VideoPlayer::Vlc => Self::Vlc(vlc::VideoPlayer::new(
                playlist
                    .header
                    .video_player_path
                    .expect("Video player path not found"),
                player.current_path(),
                ctx,
            )),
            _ => panic!("No video player selected"),
        }
    }

    fn update(&mut self, ui: &mut Ui, ctx: &Context) {
        match self {
            VideoPlayer::Native(video_player) => video_player.update(ui, ctx),
            VideoPlayer::Vlc(video_player) => video_player.update(ui, ctx),
        }
    }

    fn volume(&self) -> u8 {
        match self {
            VideoPlayer::Native(video_player) => video_player.volume(),
            VideoPlayer::Vlc(video_player) => video_player.volume(),
        }
    }

    fn set_volume(&mut self, volume: u8) -> Result<()> {
        match self {
            VideoPlayer::Native(video_player) => video_player.set_volume(volume),
            VideoPlayer::Vlc(video_player) => video_player.set_volume(volume),
        }
    }

    fn position(&self) -> u32 {
        match self {
            VideoPlayer::Native(video_player) => video_player.position(),
            VideoPlayer::Vlc(video_player) => video_player.position(),
        }
    }

    fn duration(&self) -> u32 {
        match self {
            VideoPlayer::Native(video_player) => video_player.duration(),
            VideoPlayer::Vlc(video_player) => video_player.duration(),
        }
    }

    fn seek(&mut self, position: u32) -> Result<()> {
        match self {
            VideoPlayer::Native(video_player) => video_player.seek(position),
            VideoPlayer::Vlc(video_player) => video_player.seek(position),
        }
    }
    fn fast_forward(&mut self, seconds: u32) -> Result<()> {
        match self {
            VideoPlayer::Native(video_player) => video_player.fast_forward(seconds),
            VideoPlayer::Vlc(video_player) => video_player.fast_forward(seconds),
        }
    }
    fn rewind(&mut self, seconds: u32) -> Result<()> {
        match self {
            VideoPlayer::Native(video_player) => video_player.rewind(seconds),
            VideoPlayer::Vlc(video_player) => video_player.rewind(seconds),
        }
    }

    fn is_paused(&self) -> bool {
        match self {
            VideoPlayer::Native(video_player) => video_player.is_paused(),
            VideoPlayer::Vlc(video_player) => video_player.is_paused(),
        }
    }

    fn pause(&mut self) -> Result<()> {
        match self {
            VideoPlayer::Native(video_player) => video_player.pause(),
            VideoPlayer::Vlc(video_player) => video_player.pause(),
        }
    }

    fn resume(&mut self) -> Result<()> {
        match self {
            VideoPlayer::Native(video_player) => video_player.resume(),
            VideoPlayer::Vlc(video_player) => video_player.resume(),
        }
    }
}

pub struct MediaPlayer {
    volume_icon: ButtonIcon,

    error: VecDeque<String>,

    gl: Arc<glow::Context>,

    video_player: VideoPlayer,
}

impl MediaPlayer {
    pub fn new(player: &Player, ctx: &Context, gl: Arc<glow::Context>) -> Self {
        let video_player = VideoPlayer::new(player, ctx, gl.clone());

        Self {
            volume_icon: ButtonIcon::from_rgba_image_files(
                "volume",
                Path::new(CLI.assets_path.as_str())
                    .join("image")
                    .join("icon")
                    .join("volume.png"),
                ctx,
            ),
            video_player,
            error: VecDeque::new(),
            gl,
        }
    }
    pub fn update(&mut self, ui: &mut egui::Ui, ctx: &egui::Context, can_input: bool) {
        self.video_player.update(ui, ctx);

        ui.with_layout(Layout::left_to_right(Align::TOP), |ui| {
            ui.set_height(CONTROLLER_HEIGHT);

            self.volume_icon.show(
                Vec2::new(CONTROLLER_HEIGHT - 3.0, CONTROLLER_HEIGHT - 3.0),
                ui,
            );
            ui.style_mut().drag_value_text_style = TextStyle::Body;
            let mut volume = self.video_player.volume();
            if ui
                .add(
                    egui::DragValue::new(&mut volume)
                        .speed(1)
                        .clamp_range(0..=u8::MAX)
                        .suffix("%"),
                )
                .changed()
            {
                let _ = self.video_player.set_volume(volume);
            }

            ui.add_space(10.0);
            ui.spacing_mut().slider_width = ui.available_width() - 150.0;
            let mut position = self.video_player.position();
            let duration = self.video_player.duration();
            if ui
                .add(egui::Slider::new(&mut position, 0..=duration).show_value(false))
                .changed()
            {
                let _ = self.video_player.seek(position);
            }
            ui.with_layout(Layout::right_to_left(Align::TOP), |ui| {
                let (position_h, position_m, position_s) = seconds_to_h_m_s(position);
                let (duration_h, duration_m, duration_s) = seconds_to_h_m_s(duration);
                ui.label(gen_rich_text(
                    ctx,
                    format!(
                        "{:02}:{:02}:{:02} / {:02}:{:02}:{:02}",
                        position_h, position_m, position_s, duration_h, duration_m, duration_s
                    ),
                    TextStyle::Body,
                    None,
                ));
            });
        });

        if can_input {
            if ctx.input(|i| i.key_pressed(Key::Space)) {
                if self.video_player.is_paused() {
                    let _ = self.video_player.resume();
                } else {
                    let _ = self.video_player.pause();
                }
            }
            if ctx.input(|i| i.key_pressed(Key::F)) {
                let _ = self.video_player.fast_forward(5);
            }
            if ctx.input(|i| i.key_pressed(Key::B)) {
                let _ = self.video_player.rewind(5);
            }
        }
    }
}
