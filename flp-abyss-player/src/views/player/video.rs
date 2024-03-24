use anyhow::Result;
use eframe::{
    egui::{self, Key, Layout, TextStyle},
    emath::Align,
    epaint::Vec2,
};
use std::{collections::VecDeque, path::Path, sync::Arc};

use crate::{
    models::{config, player::Player},
    utils::{fonts::gen_rich_text, helper::seconds_to_h_m_s},
    views::widget::button_icon::ButtonIcon,
    CLI,
};

mod native;
mod vlc;

const CONTROLLER_HEIGHT: f32 = 20.0;

pub trait VideoPlayer {
    fn is_paused(&self) -> bool;
    fn is_end(&self) -> bool;
    fn position(&self) -> u32;
    fn duration(&self) -> u32;
    fn volume(&self) -> u8;
    fn show(&mut self, ui: &mut egui::Ui, ctx: &egui::Context);
    fn start(&mut self) -> Result<()>;
    fn resume(&mut self) -> Result<()>;
    fn pause(&mut self) -> Result<()>;
    fn stop(&mut self) -> Result<()>;
    fn set_volume(&mut self, percent: u8) -> Result<()>;
    fn seek(&mut self, seconds: u32) -> Result<()>;
    fn fast_forward(&mut self, seconds: u32) -> Result<()>;
    fn rewind(&mut self, seconds: u32) -> Result<()>;
}

pub struct MediaPlayer {
    volume_icon: ButtonIcon,

    video_player: Option<Box<dyn VideoPlayer>>,

    error: VecDeque<String>,

    gl: Arc<glow::Context>,
}

impl MediaPlayer {
    pub fn new(ctx: &egui::Context, gl: Arc<glow::Context>) -> Self {
        Self {
            volume_icon: ButtonIcon::from_rgba_image_files(
                "volume",
                Path::new(CLI.assets_path.as_str())
                    .join("image")
                    .join("icon")
                    .join("volume.png"),
                ctx,
            ),
            video_player: None,
            error: VecDeque::new(),
            gl,
        }
    }
}

impl super::MediaPlayer for MediaPlayer {
    fn is_loaded(&self) -> bool {
        self.video_player.is_some()
    }

    fn is_end(&self) -> bool {
        self.video_player
            .as_ref()
            .map(|v| v.is_end())
            .unwrap_or(false)
    }

    fn reload(&mut self, path: &dyn AsRef<Path>, ctx: &egui::Context, state: &Player) {
        let player = &state.playlist.header.video_player;
        let player_path = &state.playlist.header.video_player_path;
        if let Some(mut video_player) = self.video_player.take() {
            if let Err(err) = video_player.stop() {
                self.error.push_back(err.to_string());
            }
        }
        let mut video_player: Box<dyn VideoPlayer> = match player {
            config::VideoPlayer::Native => {
                Box::new(native::VideoPlayer::new(path, self.gl.clone(), ctx.clone()))
            }
            config::VideoPlayer::Vlc => Box::new(vlc::VideoPlayer::new(
                player_path
                    .as_ref()
                    .expect("Player path should be available at this point"),
                path,
                ctx.clone(),
            )),
            _ => panic!("Unknown video player: {:?}", player),
        };
        if let Err(err) = video_player.start() {
            self.error.push_back(err.to_string());
        }
        self.video_player.replace(video_player);
    }

    fn sync(&mut self, _paths: &Player) {}

    fn show_central_panel(&mut self, ui: &mut egui::Ui, ctx: &egui::Context, can_input: bool) {
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
