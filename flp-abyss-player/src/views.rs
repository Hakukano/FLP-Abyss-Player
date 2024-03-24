use eframe::egui;
use serde::Serialize;
use serde_json::Value;
use std::{
    process::exit,
    sync::{
        mpsc::{channel, Receiver, Sender, TryRecvError},
        Arc,
    },
};
use timer::Signal;

use crate::{library, models::config::Config};

mod config;
mod player;
mod timer;
mod widget;

trait View {
    fn handle(&mut self, packet: Packet);

    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame);
}

pub struct Task {
    packet_rx: Receiver<Packet>,
    packet_tx: Sender<Packet>,
    command_tx: Sender<Command>,
    signal_tx: Sender<Signal>,
    view: Box<dyn View>,
    gl: Arc<glow::Context>,
}

impl Task {
    fn new(
        packet_rx: Receiver<Packet>,
        packet_tx: Sender<Packet>,
        command_tx: Sender<Command>,
        signal_tx: Sender<Signal>,
        cc: &eframe::CreationContext<'_>,
    ) -> Self {
        library::fonts::init(&cc.egui_ctx);
        Self {
            view: Box::new(config::View::new(
                Config::all(),
                command_tx.clone(),
                &cc.egui_ctx,
            )),
            packet_rx,
            packet_tx,
            command_tx,
            signal_tx,
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

                let (signal_tx, signal_rx) = channel::<Signal>();
                let _ = timer::Task::run(signal_rx, packet_tx.clone(), cc.egui_ctx.clone());
                let task = Task::new(packet_rx, packet_tx, command_tx, signal_tx, cc);
                Box::new(task)
            }),
        );
    }
}

impl eframe::App for Task {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        match self.packet_rx.try_recv() {
            Err(TryRecvError::Disconnected) => exit(500),
            Ok(packet) => {
                if let PacketName::ChangeView(view) = packet.name {
                    match view {
                        ViewType::Config => {
                            self.view = Box::new(config::View::new(
                                serde_json::from_value(packet.data).unwrap(),
                                self.command_tx.clone(),
                                ctx,
                            ))
                        }
                        ViewType::Player => {
                            self.view = Box::new(player::View::new(
                                serde_json::from_value(packet.data).unwrap(),
                                self.packet_tx.clone(),
                                self.command_tx.clone(),
                                self.signal_tx.clone(),
                                ctx,
                                self.gl.clone(),
                            ))
                        }
                    }
                    return;
                }

                self.view.handle(packet);
            }
            _ => {}
        }

        self.view.update(ctx, frame);
    }
}
