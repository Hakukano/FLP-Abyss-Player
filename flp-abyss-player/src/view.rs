use eframe::egui;
use serde_json::Value;

use serde::{Deserialize, Serialize};
#[cfg(feature = "native")]
use std::sync::Arc;
use std::{
    process::exit,
    sync::mpsc::{Receiver, Sender, TryRecvError},
};

use crate::{controller::Command, library, model::config::Config};

mod config;
mod player;
mod widget;

#[derive(Eq, PartialEq, Hash, Deserialize, Serialize)]
pub enum ViewType {
    Config,
    Player,
}

#[derive(Eq, PartialEq)]
pub enum PacketName {
    ChangeView(ViewType),
    Update,
    Filter,
}

pub struct Packet {
    pub name: PacketName,
    pub data: Value,
}

impl Packet {
    pub fn new(name: PacketName, data: Value) -> Self {
        Self { name, data }
    }
}

trait View: Send + Sync {
    fn handle(&mut self, packet: Packet);

    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame);
}

pub struct Task {
    packet_rx: Receiver<Packet>,
    packet_tx: Sender<Packet>,
    command_tx: Sender<Command>,
    view: Box<dyn View>,

    #[cfg(feature = "native")]
    gl: Arc<glow::Context>,
}

impl Task {
    fn new(
        packet_rx: Receiver<Packet>,
        packet_tx: Sender<Packet>,
        command_tx: Sender<Command>,
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
            #[cfg(feature = "native")]
            gl: cc.gl.clone().expect("gl context should be available"),
        }
    }

    pub fn run(
        packet_rx: Receiver<Packet>,
        packet_tx: Sender<Packet>,
        command_tx: Sender<Command>,
    ) {
        let options = eframe::NativeOptions {
            initial_window_size: Some(egui::vec2(1600.0, 900.0)),
            #[cfg(feature = "native")]
            multisampling: 4,
            #[cfg(feature = "native")]
            renderer: eframe::Renderer::Glow,
            ..Default::default()
        };
        let _ = eframe::run_native(
            t!("ui.app_name").as_str(),
            options,
            Box::new(|cc| {
                let task = Task::new(packet_rx, packet_tx, command_tx, cc);
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
