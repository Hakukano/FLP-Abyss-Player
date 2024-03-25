use chrono::{DateTime, Utc};
use eframe::egui::Context;
use std::{
    process::exit,
    sync::mpsc::{Receiver, Sender, TryRecvError},
    thread,
    thread::JoinHandle,
    time,
};

pub enum TimerSignal {
    Start(u32),
    Update(u32),
    Stop,
}

pub struct Task {
    timer_signal: Receiver<TimerSignal>,
    tick_signal: Sender<()>,

    running: bool,
    interval: u32,
    last_triggered: DateTime<Utc>,
}

impl Task {
    fn new(timer_signal: Receiver<TimerSignal>, tick_signal: Sender<()>) -> Self {
        Self {
            timer_signal,
            tick_signal,
            running: false,
            interval: 0,
            last_triggered: Utc::now(),
        }
    }

    pub fn run(
        timer_signal: Receiver<TimerSignal>,
        tick_signal: Sender<()>,
        ctx: Context,
    ) -> JoinHandle<()> {
        let mut task = Self::new(timer_signal, tick_signal);
        thread::spawn(move || loop {
            match task.timer_signal.try_recv() {
                Err(TryRecvError::Disconnected) => exit(500),
                Ok(signal) => match signal {
                    TimerSignal::Start(interval) => {
                        task.running = true;
                        task.interval = interval;
                        task.last_triggered = Utc::now();
                    }
                    TimerSignal::Update(interval) => {
                        task.interval = interval;
                    }
                    TimerSignal::Stop => {
                        task.running = false;
                    }
                },
                _ => {}
            }

            let delta = Utc::now() - task.last_triggered;
            let overshot = delta.num_milliseconds() - (task.interval as i64 * 1000);
            if !task.running || (overshot < 0) {
                thread::sleep(time::Duration::from_millis(20));
                continue;
            }

            task.tick_signal.send(()).unwrap();

            task.last_triggered = Utc::now()
                - chrono::Duration::try_milliseconds(overshot)
                    .unwrap_or_else(chrono::Duration::zero);

            ctx.request_repaint();
        })
    }
}
