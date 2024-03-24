use eframe::egui;
use serde_json::Value;
use std::{
    collections::HashMap,
    sync::{
        mpsc::{channel, Receiver, Sender, TryRecvError},
        Arc,
    },
};
use timer::TimerSignal;

use crate::utils;

mod config;
mod init;
mod player;
mod timer;
mod widgets;

struct ChangeLocation {
    path: Vec<String>,
    query: HashMap<String, Value>,
}

enum View {
    Init(init::View),
    Config(config::View),
}

pub struct Task {
    change_location_tx: Sender<ChangeLocation>,
    change_location_rx: Receiver<ChangeLocation>,
    tick_signal_rx: Receiver<()>,
    timer_signal_tx: Sender<TimerSignal>,
    gl: Arc<glow::Context>,

    view: View,
    location: Vec<String>,
}

impl Task {
    fn new(
        change_location_tx: Sender<ChangeLocation>,
        change_location_rx: Receiver<ChangeLocation>,
        tick_signal_rx: Receiver<()>,
        timer_signal_tx: Sender<TimerSignal>,
        cc: &eframe::CreationContext<'_>,
        view: View,
    ) -> Self {
        utils::fonts::init(&cc.egui_ctx);
        Self {
            change_location_tx,
            change_location_rx,
            tick_signal_rx,
            timer_signal_tx,
            gl: cc.gl.clone().expect("gl context should be available"),

            view,
            location: vec!["configs".to_string(), "default".to_string()],
        }
    }

    pub fn run() {
        let options = eframe::NativeOptions {
            viewport: egui::ViewportBuilder::default().with_inner_size(egui::vec2(1600.0, 900.0)),
            multisampling: 4,
            renderer: eframe::Renderer::Glow,
            ..Default::default()
        };
        let _ = eframe::run_native(
            t!("ui.app_name").as_ref(),
            options,
            Box::new(|cc| {
                egui_extras::install_image_loaders(&cc.egui_ctx);

                let (change_location_tx, change_location_rx) = channel::<ChangeLocation>();
                let (timer_signal_tx, timer_signal_rx) = channel::<TimerSignal>();
                let (tick_signal_tx, tick_signal_rx) = channel::<()>();

                let _ =
                    timer::Task::run(timer_signal_rx, tick_signal_tx.clone(), cc.egui_ctx.clone());
                let task = Task::new(
                    change_location_tx.clone(),
                    change_location_rx,
                    tick_signal_rx,
                    timer_signal_tx,
                    cc,
                    View::Init(init::View::new(change_location_tx)),
                );
                Box::new(task)
            }),
        );
    }
}

impl eframe::App for Task {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        match self.change_location_rx.try_recv() {
            Ok(ChangeLocation { path, query }) => match path
                .iter()
                .map(AsRef::as_ref)
                .collect::<Vec<_>>()
                .as_slice()
            {
                ["configs", id] => {}
            },
            _ => {}
        }

        let need_to_tick = match self.tick_signal_rx.try_recv() {
            Err(TryRecvError::Disconnected) => panic!("Timer exited before the app"),
            Ok(()) => true,
            _ => false,
        };

        match &mut self.view {
            View::Init(view) => view.update(),
            View::Config(view) => {}
            _ => {}
        }
    }
}
