use serde_json::Value;
use std::{
    collections::HashMap,
    sync::mpsc::{Receiver, Sender},
    thread,
    thread::JoinHandle,
};

use crate::view::Packet;

mod config;
mod player;

#[derive(Eq, PartialEq, Hash)]
pub enum ControllerType {
    Config,
    Player,
}

#[derive(Eq, PartialEq)]
pub enum CommandName {
    Terminate,
    Update,
    Reload,
    Search,
    Save,
    Load,
}

pub struct Command {
    target: ControllerType,
    pub name: CommandName,
    pub arguments: Value,
}

impl Command {
    pub fn new(target: ControllerType, name: CommandName, arguments: Value) -> Self {
        Self {
            target,
            name,
            arguments,
        }
    }
}

trait Controller: Send + Sync {
    fn handle(&mut self, command: Command);
}

pub struct Task {
    command_rx: Receiver<Command>,
    controllers: HashMap<ControllerType, Box<dyn Controller>>,
}

impl Task {
    fn new(command_rx: Receiver<Command>, packet_tx: Sender<Packet>) -> Self {
        Self {
            command_rx,
            controllers: HashMap::from([
                (
                    ControllerType::Config,
                    Box::new(config::Controller::new(packet_tx.clone())) as Box<dyn Controller>,
                ),
                (
                    ControllerType::Player,
                    Box::new(player::Controller::new(packet_tx.clone())) as Box<dyn Controller>,
                ),
            ]),
        }
    }

    pub fn run(command_rx: Receiver<Command>, packet_tx: Sender<Packet>) -> JoinHandle<()> {
        let mut task = Task::new(command_rx, packet_tx);
        thread::spawn(move || {
            for command in task.command_rx.iter() {
                if command.name == CommandName::Terminate {
                    break;
                }

                task.controllers
                    .get_mut(&command.target)
                    .unwrap()
                    .handle(command);
            }
        })
    }
}
