use chrono::{DateTime, Utc};
use eframe::egui::Context;
use serde_json::Value;
use std::{
    process::exit,
    sync::mpsc::{Receiver, Sender, TryRecvError},
    thread,
    thread::JoinHandle,
    time,
};

use crate::view::{Packet, PacketName};

pub enum SignalName {
    Start,
    Stop,
}

pub struct Signal {
    pub name: SignalName,
    pub data: Value,
}

impl Signal {
    pub fn new(name: SignalName, data: Value) -> Self {
        Self { name, data }
    }
}

pub struct Task {
    signal_rx: Receiver<Signal>,
    packet_tx: Sender<Packet>,

    running: bool,
    interval: u32,
    last_triggered: DateTime<Utc>,
}

impl Task {
    fn new(signal_rx: Receiver<Signal>, packet_tx: Sender<Packet>) -> Self {
        Self {
            signal_rx,
            packet_tx,
            running: false,
            interval: 0,
            last_triggered: Utc::now(),
        }
    }

    pub fn run(
        signal_rx: Receiver<Signal>,
        packet_tx: Sender<Packet>,
        ctx: Context,
    ) -> JoinHandle<()> {
        let mut task = Self::new(signal_rx, packet_tx);
        thread::spawn(move || loop {
            match task.signal_rx.try_recv() {
                Err(TryRecvError::Disconnected) => exit(500),
                Ok(signal) => match signal.name {
                    SignalName::Start => {
                        task.running = true;
                        task.interval = serde_json::from_value(signal.data).unwrap();
                        task.last_triggered = Utc::now();
                    }
                    SignalName::Stop => {
                        task.running = false;
                    }
                },
                _ => {}
            }

            let delta = Utc::now() - task.last_triggered;
            let overshot = delta.num_milliseconds() - (task.interval as i64 * 1000);
            if !task.running || (overshot < 0) {
                thread::sleep(time::Duration::from_millis(50));
                continue;
            }

            task.packet_tx
                .send(Packet::new(PacketName::Tick, Value::Null))
                .unwrap();

            task.last_triggered = Utc::now() - chrono::Duration::milliseconds(overshot);

            ctx.request_repaint();
        })
    }
}
