use eframe::{
    egui::{self, Key, Layout, TextStyle},
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
    Native,
    Vlc,
}

impl VideoPlayer {
    fn new(player: &Player) -> Self {
        match player
            .playlist()
            .expect("Playlist not found")
            .header
            .video_player
        {
            config::VideoPlayer::Native => Self::Native,
            config::VideoPlayer::Vlc => Self::Vlc,
            _ => panic!("No video player selected"),
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
    pub fn new(player: &Player, ctx: &egui::Context, gl: Arc<glow::Context>) -> Self {
        let video_player = VideoPlayer::new(player);

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
        if let Some(video_player) = self.video_player.as_mut() {
            video_player.show(ui, ctx);

            ui.with_layout(Layout::left_to_right(Align::TOP), |ui| {
                ui.set_height(CONTROLLER_HEIGHT);

                self.volume_icon.show(
                    Vec2::new(CONTROLLER_HEIGHT - 3.0, CONTROLLER_HEIGHT - 3.0),
                    ui,
                );
                ui.style_mut().drag_value_text_style = TextStyle::Body;
                let mut volume = video_player.volume();
                if ui
                    .add(
                        egui::DragValue::new(&mut volume)
                            .speed(1)
                            .clamp_range(0..=u8::MAX)
                            .suffix("%"),
                    )
                    .changed()
                {
                    let _ = video_player.set_volume(volume);
                }

                ui.add_space(10.0);
                ui.spacing_mut().slider_width = ui.available_width() - 150.0;
                let mut position = video_player.position();
                let duration = video_player.duration();
                if ui
                    .add(egui::Slider::new(&mut position, 0..=duration).show_value(false))
                    .changed()
                {
                    let _ = video_player.seek(position);
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
                    if video_player.is_paused() {
                        let _ = video_player.resume();
                    } else {
                        let _ = video_player.pause();
                    }
                }
                if ctx.input(|i| i.key_pressed(Key::F)) {
                    let _ = video_player.fast_forward(5);
                }
                if ctx.input(|i| i.key_pressed(Key::B)) {
                    let _ = video_player.rewind(5);
                }
            }
        }
    }
}
