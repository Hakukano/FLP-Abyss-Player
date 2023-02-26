use std::{
    path::Path,
    process::{Child, Command},
};

use anyhow::Result;
use chrono::{DateTime, TimeZone, Utc};
use passwords::PasswordGenerator;
use serde::Deserialize;
use tokio::runtime;

use crate::helper::find_available_port;

const VLC_HTTP_HOST: &str = "127.0.0.1";
const RUNTIME_THREADS: usize = 4;
const RUNTIME_THREAD_NAME: &str = "video_player";
const RUNTIME_THREAD_STACK_SIZE: usize = 3 * 1024 * 1024;

#[derive(Default, Deserialize)]
#[serde(rename = "root")]
struct Status {
    fullscreen: bool,
}

pub struct VideoPlayer {
    command: Command,
    http_password: String,
    child: Option<Child>,

    runtime: runtime::Runtime,

    status: Status,
    last_sync: DateTime<Utc>,
}

impl VideoPlayer {
    pub fn new(player_path: impl AsRef<Path>, video_path: impl AsRef<Path>) -> Self {
        let pg = PasswordGenerator {
            length: 16,
            numbers: true,
            lowercase_letters: true,
            uppercase_letters: true,
            symbols: false,
            spaces: false,
            exclude_similar_characters: false,
            strict: true,
        };
        let http_password = pg.generate_one().expect("Cannot generate password");
        let mut command = Command::new(player_path.as_ref());
        command
            .arg("--extraintf")
            .arg("http")
            .arg("--http-host")
            .arg(VLC_HTTP_HOST)
            .arg("--http-port")
            .arg(
                find_available_port()
                    .expect("Cannot find an available port")
                    .to_string(),
            )
            .arg("--http-password")
            .arg(http_password.as_str())
            .arg(video_path.as_ref());
        let runtime = runtime::Builder::new_multi_thread()
            .worker_threads(RUNTIME_THREADS)
            .thread_name(RUNTIME_THREAD_NAME)
            .thread_stack_size(RUNTIME_THREAD_STACK_SIZE)
            .build()
            .expect("Cannot build tokio runtime");
        Self {
            command,
            http_password,
            child: None,
            runtime,
            status: Status::default(),
            last_sync: Utc.timestamp_opt(0, 0).unwrap(),
        }
    }

    fn sync_status(&mut self) {}
}

impl super::VideoPlayer for VideoPlayer {
    fn show(&mut self, ui: &mut eframe::egui::Ui, ctx: &eframe::egui::Context) {}

    fn start(&mut self) -> Result<()> {
        if self.child.is_none() {
            self.child.replace(self.command.spawn()?);
        }
        Ok(())
    }

    fn stop(&mut self) -> Result<()> {
        if let Some(mut child) = self.child.take() {
            child.kill()?;
        }
        Ok(())
    }
}
