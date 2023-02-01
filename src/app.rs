mod config;
mod view;

use eframe::egui;

enum State {
    Config,
    View(view::State),
}

impl Default for State {
    fn default() -> Self {
        Self::Config
    }
}

#[derive(Default)]
pub struct App {
    state: State,
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let mut next_state = None;
        match &mut self.state {
            State::Config => {
                let should_go = {
                    let mut config = config::get().lock().expect("Cannot get config lock");
                    config.update(ctx);
                    let should_go = config.should_go();
                    if should_go {
                        config.reset();
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
                    next_state.replace(State::Config);
                    state.reset();
                }
            }
        }
        if let Some(next_state) = next_state {
            self.state = next_state;
        }
    }
}
