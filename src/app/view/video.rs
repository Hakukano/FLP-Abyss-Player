#[cfg(feature = "native")]
mod native;
mod vlc;

#[cfg(feature = "native")]
use std::sync::Arc;
use std::{collections::VecDeque, path::Path};

use anyhow::Result;
use eframe::egui::{self, Key};

use crate::config;

pub trait VideoPlayer: Send + Sync {
    fn is_paused(&self) -> bool;
    fn is_end(&self) -> bool;
    fn show(&mut self, ui: &mut egui::Ui, ctx: &egui::Context);
    fn start(&mut self) -> Result<()>;
    fn resume(&mut self) -> Result<()>;
    fn pause(&mut self) -> Result<()>;
    fn stop(&mut self) -> Result<()>;
    fn seek(&mut self, seconds: u32) -> Result<()>;
    fn fast_forward(&mut self, seconds: u32) -> Result<()>;
    fn rewind(&mut self, seconds: u32) -> Result<()>;
}

pub struct MediaPlayer {
    video_player: Option<Box<dyn VideoPlayer>>,

    error: VecDeque<String>,

    #[cfg(feature = "native")]
    gl: Arc<glow::Context>,
}

impl MediaPlayer {
    pub fn new(#[cfg(feature = "native")] gl: Arc<glow::Context>) -> Self {
        Self {
            video_player: None,
            error: VecDeque::new(),
            #[cfg(feature = "native")]
            gl,
        }
    }
}

impl super::MediaPlayer for MediaPlayer {
    fn support_extensions(&self) -> &[&str] {
        &["mp4", "mov"]
    }

    fn is_loaded(&self) -> bool {
        self.video_player.is_some()
    }

    fn is_end(&self) -> bool {
        self.video_player
            .as_ref()
            .map(|v| v.is_end())
            .unwrap_or(false)
    }

    fn reload(&mut self, path: &dyn AsRef<Path>, ctx: &egui::Context) {
        let (player, player_path) = {
            let config = config::get().read().expect("Cannot get config lock");
            (config.video_player, config.video_player_path.clone())
        };
        if let Some(mut video_player) = self.video_player.take() {
            if let Err(err) = video_player.stop() {
                self.error.push_back(err.to_string());
            }
        }
        let mut video_player: Box<dyn VideoPlayer> = match player {
            #[cfg(feature = "native")]
            config::VideoPlayer::Native => {
                Box::new(native::VideoPlayer::new(path, self.gl.clone(), ctx.clone()))
            }
            config::VideoPlayer::Vlc => Box::new(vlc::VideoPlayer::new(
                player_path.expect("Player path should be availalbe at this point"),
                path,
                ctx.clone(),
            )),
            _ => panic!("Unknow video player: {:?}", player),
        };
        if let Err(err) = video_player.start() {
            self.error.push_back(err.to_string());
        }
        self.video_player.replace(video_player);
    }

    fn show_central_panel(&mut self, ui: &mut egui::Ui, ctx: &egui::Context, can_input: bool) {
        if let Some(video_player) = self.video_player.as_mut() {
            video_player.show(ui, ctx);

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
