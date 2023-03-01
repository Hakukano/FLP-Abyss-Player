mod vlc;

use std::{collections::VecDeque, path::Path};

use anyhow::Result;
use eframe::egui::{self, Key};

use crate::config;

pub trait VideoPlayer: Send + Sync {
    fn is_end(&self) -> bool;
    fn show(&mut self, ui: &mut egui::Ui, ctx: &egui::Context);
    fn start(&mut self) -> Result<()>;
    fn pause(&mut self) -> Result<()>;
    fn stop(&mut self) -> Result<()>;
    fn fast_forward(&mut self, seconds: u32) -> Result<()>;
    fn rewind(&mut self, seconds: u32) -> Result<()>;
}

pub struct MediaPlayer {
    support_extensions: Vec<String>,

    video_player: Option<Box<dyn VideoPlayer>>,

    error: VecDeque<String>,
}

impl MediaPlayer {
    pub fn new() -> Self {
        Self {
            support_extensions: vec![
                "avi".to_string(),
                "mov".to_string(),
                "mp4".to_string(),
                "wmv".to_string(),
            ],
            video_player: None,
            error: VecDeque::new(),
        }
    }
}

impl super::MediaPlayer for MediaPlayer {
    fn support_extensions(&self) -> &[String] {
        self.support_extensions.as_slice()
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
            (
                config.video_player,
                config
                    .video_player_path
                    .clone()
                    .expect("video player path should have be set at this point"),
            )
        };
        if let Some(mut video_player) = self.video_player.take() {
            if let Err(err) = video_player.stop() {
                self.error.push_back(err.to_string());
            }
        }
        let mut video_player: Box<dyn VideoPlayer> = match player {
            config::VideoPlayer::Vlc => {
                Box::new(vlc::VideoPlayer::new(player_path, path, ctx.clone()))
            }
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
                    let _ = video_player.pause();
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
