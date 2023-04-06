use std::{
    path::PathBuf,
    sync::{Arc, RwLock},
};

use tokio::runtime::{self, Runtime};

use self::service::playlist::memory::Playlist;

mod client;
mod service;

const BIND_HTTP_HOST: &str = "127.0.0.1";
const RUNTIME_THREADS: usize = 4;
const RUNTIME_THREAD_NAME: &str = "server_player";
const RUNTIME_THREAD_STACK_SIZE: usize = 3 * 1024 * 1024;

pub struct MediaPlayer {
    paths: Arc<RwLock<Vec<PathBuf>>>,

    runtime: Runtime,
}

impl MediaPlayer {
    pub fn new() -> Self {
        let runtime = runtime::Builder::new_multi_thread()
            .worker_threads(RUNTIME_THREADS)
            .thread_name(RUNTIME_THREAD_NAME)
            .thread_stack_size(RUNTIME_THREAD_STACK_SIZE)
            .enable_all()
            .build()
            .expect("Cannot build tokio runtime");
        let slf = Self {
            paths: Arc::new(RwLock::new(Vec::new())),
            runtime,
        };

        let paths = slf.paths.clone();
        slf.runtime.spawn(async move {
            let playlist = Arc::new(Playlist { paths });
        });
        slf
    }
}

impl super::MediaPlayer for MediaPlayer {
    fn is_loaded(&self) -> bool {
        true
    }

    fn is_end(&self) -> bool {
        false
    }

    fn support_extensions(&self) -> &[&str] {
        &[
            "bmp", "gif", "jpeg", "jpg", "png", "avi", "mkv", "mov", "mp4", "webm", "mp3", "wav",
            "flac",
        ]
    }

    fn reload(&mut self, _path: &dyn AsRef<std::path::Path>, _ctx: &eframe::egui::Context) {}

    fn sync(&mut self, paths: &[PathBuf]) {
        *self.paths.write().unwrap() = paths.to_vec();
    }

    fn show_central_panel(
        &mut self,
        ui: &mut eframe::egui::Ui,
        ctx: &eframe::egui::Context,
        can_input: bool,
    ) {
    }
}
