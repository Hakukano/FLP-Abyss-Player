use eframe::egui;
use std::{
    process::exit,
    sync::{
        mpsc::{channel, Receiver, Sender, TryRecvError},
        Arc,
    },
};
use timer::TimerSignal;

use crate::utils;

mod config;
mod player;
mod timer;
mod widgets;

trait View {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame);
}

pub struct Task {
    tick_signal_rx: Receiver<()>,
    timer_signal_tx: Sender<TimerSignal>,
    view: Box<dyn View>,
    gl: Arc<glow::Context>,
}

impl Task {
    fn new(
        tick_signal_rx: Receiver<()>,
        timer_signal_tx: Sender<TimerSignal>,
        cc: &eframe::CreationContext<'_>,
    ) -> Self {
        utils::fonts::init(&cc.egui_ctx);
        Self {
            view: Box::new(),
            tick_signal_rx,
            timer_signal_tx,
            gl: cc.gl.clone().expect("gl context should be available"),
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

                let (timer_signal_tx, timer_signal_rx) = channel::<TimerSignal>();
                let (tick_signal_tx, tick_signal_rx) = channel::<()>();

                let _ =
                    timer::Task::run(timer_signal_rx, tick_signal_tx.clone(), cc.egui_ctx.clone());
                let task = Task::new(tick_signal_rx, timer_signal_tx, cc);
                Box::new(task)
            }),
        );
    }
}

impl eframe::App for Task {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        match self.tick_signal_rx.try_recv() {
            Err(TryRecvError::Disconnected) => exit(500),
            Ok(()) => {
                if let PacketName::ChangeView(view) = tick_signal.name {
                    match view {
                        ViewType::Config => {
                            self.view = Box::new(config::View::new(
                                serde_json::from_value(tick_signal.data).unwrap(),
                                self.command_tx.clone(),
                                ctx,
                            ))
                        }
                        ViewType::Player => {
                            self.view = Box::new(player::View::new(
                                serde_json::from_value(tick_signal.data).unwrap(),
                                self.tick_signal_tx.clone(),
                                self.command_tx.clone(),
                                self.timer_signal_tx.clone(),
                                ctx,
                                self.gl.clone(),
                            ))
                        }
                    }
                    return;
                }

                self.view.handle(tick_signal);
            }
            _ => {}
        }

        self.view.update(ctx, frame);
    }
}
