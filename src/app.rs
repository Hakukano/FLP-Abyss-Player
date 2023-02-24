mod config;
mod view;

use eframe::egui;

enum State {
    Config(config::State),
    View(view::State),
}

pub struct App {
    state: State,
}

impl App {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        crate::font::init(&cc.egui_ctx);
        Self {
            state: State::Config(config::State::default()),
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let mut next_state = None;
        match &mut self.state {
            State::Config(state) => {
                let should_go = {
                    state.update(ctx);
                    let should_go = state.should_go();
                    if should_go {
                        state.reset();
                    }
                    should_go
                };

                if should_go {
                    next_state.replace(State::View(view::State::new()));
                }
            }
            State::View(state) => {
                state.update(ctx);
                if state.should_home() {
                    next_state.replace(State::Config(config::State::default()));
                    state.reset();
                }
            }
        }
        if let Some(next_state) = next_state {
            self.state = next_state;
        }
    }
}
