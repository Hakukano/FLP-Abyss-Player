use std::{path::PathBuf, sync::Arc};

use eframe::{egui::TextStyle, epaint::Color32};
use parking_lot::RwLock;
use tokio::runtime::{self, Runtime};

use crate::{
    library::{fonts::gen_rich_text, helper::find_available_port},
    model::player::Player,
};

use self::{client::http_server::HttpServer, service::playlist::memory::Playlist};

mod client;
mod service;

const BIND_HOST: &str = "0.0.0.0";
const RUNTIME_THREADS: usize = 4;
const RUNTIME_THREAD_NAME: &str = "server_player";
const RUNTIME_THREAD_STACK_SIZE: usize = 3 * 1024 * 1024;

pub struct MediaPlayer {
    paths: Arc<RwLock<Vec<PathBuf>>>,

    bind_port: u16,

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
            bind_port: find_available_port().expect("Cannot find available port"),
            runtime,
        };

        let paths = slf.paths.clone();
        let bind_port = slf.bind_port;
        slf.runtime.spawn(async move {
            let playlist = Arc::new(Playlist { paths });

            let http_server = HttpServer { playlist };
            http_server.run(BIND_HOST, bind_port).await
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

    fn reload(
        &mut self,
        _path: &dyn AsRef<std::path::Path>,
        _ctx: &eframe::egui::Context,
        _state: &Player,
    ) {
    }

    fn sync(&mut self, state: &Player) {
        *self.paths.write() = state
            .playlist
            .body
            .item_paths
            .iter()
            .map(PathBuf::from)
            .collect();
    }

    fn show_central_panel(
        &mut self,
        ui: &mut eframe::egui::Ui,
        ctx: &eframe::egui::Context,
        _can_input: bool,
    ) {
        ui.label(gen_rich_text(
            ctx,
            format!("Listening on: {}:{}", BIND_HOST, self.bind_port),
            TextStyle::Body,
            Some(Color32::LIGHT_YELLOW),
        ));
    }
}
