use std::{
    path::PathBuf,
    sync::{Arc, RwLock},
};

use tokio::runtime::{self, Runtime};

use crate::helper::find_available_port;

use self::{client::http_server::HttpServer, service::playlist::memory::Playlist};

mod client;
mod service;

const BIND_HOST: &str = "0.0.0.0";
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

            let http_server = HttpServer { playlist };
            http_server
                .run(
                    BIND_HOST,
                    find_available_port().expect("Cannot find available port"),
                )
                .await
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
            "bmp", "gif", "jpeg", "jpg", "png", "avi", "mp4", "webm", "mp3", "wav",
        ]
    }

    fn reload(&mut self, _path: &dyn AsRef<std::path::Path>, _ctx: &eframe::egui::Context) {}

    fn sync(&mut self, paths: &[PathBuf]) {
        *self.paths.write().unwrap() = paths.to_vec();
    }

    fn show_central_panel(
        &mut self,
        _ui: &mut eframe::egui::Ui,
        _ctx: &eframe::egui::Context,
        _can_input: bool,
    ) {
    }
}
