mod config;
mod view;

use eframe::egui;

enum State {
    Config(Box<config::State>),
    View(view::TimedState),
}

pub struct App {
    state: State,
}

impl App {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        crate::font::init(&cc.egui_ctx);
        Self {
            state: State::Config(Box::new(config::State::new(&cc.egui_ctx))),
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
                    state.should_go()
                };

                if should_go {
                    next_state.replace(State::View(view::TimedState::new(
                        view::State::new(ctx),
                        ctx.clone(),
                    )));
                }
            }
            State::View(timed_state) => {
                timed_state
                    .state
                    .write()
                    .expect("Cannot get view state lock")
                    .update(ctx);
                if timed_state
                    .state
                    .read()
                    .expect("Cannot get view state lock")
                    .should_home()
                {
                    next_state.replace(State::Config(Box::new(config::State::new(ctx))));
                }
            }
        }
        if let Some(next_state) = next_state {
            self.state = next_state;
        }
    }
}
