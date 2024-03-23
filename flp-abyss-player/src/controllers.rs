use serde::Serialize;
use serde_json::Value;
use std::{
    sync::mpsc::{Receiver, Sender},
    thread,
    thread::JoinHandle,
};

use crate::views::Packet;

mod config;
mod player;

#[derive(Clone, Copy)]
pub enum Method {
    Create,
    Get,
    Update,
    Delete,
    Custom(String),
}

#[derive(Debug)]
pub struct Command {
    path: Vec<String>,
    method: Method,
    body: Value,
}

impl Command {
    pub fn new<Body>(path: Vec<String>, method: Method, body: Body) -> Self
    where
        Body: Serialize,
    {
        Self {
            path,
            method,
            body: serde_json::to_value(body).expect("Cannot serialize command body"),
        }
    }
}

pub fn run(command_rx: Receiver<Command>, packet_tx: Sender<Packet>) -> JoinHandle<()> {
    thread::spawn(move || {
        for command in command_rx.iter() {
            let packet = match command.path.iter().map(AsRef::as_ref).collect().as_slice() {
                ["configs"] => match command.method {
                    Method::Get => config::index(),
                    _ => panic!("Unknown method: {:?}", command.method),
                },
                ["configs", id] => match command.method.as_ref() {
                    Method::Get => config::get(id),
                    _ => panic!("Unknown method: {:?}", command.method),
                },
                _ => panic!("Unknown command: {:?}", command),
            };
            if packet_tx.send(packet).is_err() {
                break;
            }
        }
    })
}
