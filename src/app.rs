mod config;
pub mod view;

#[cfg(feature = "opengl")]
use std::sync::Arc;

use eframe::egui;

enum State {
    Config(Box<config::State>),
    View(view::TimedState),
}

pub struct App {
    state: State,

    #[cfg(feature = "opengl")]
    gl: Arc<glow::Context>,
}

impl App {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        crate::font::init(&cc.egui_ctx);
        Self {
            state: State::Config(Box::new(config::State::new(&cc.egui_ctx))),
            #[cfg(feature = "opengl")]
            gl: cc.gl.clone().expect("gl context should be availble"),
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
                        view::State::new(
                            ctx,
                            state.playlist.as_ref(),
                            #[cfg(feature = "opengl")]
                            self.gl.clone(),
                        ),
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
                    crate::config::get()
                        .write()
                        .expect("Cannot get config lock")
                        .playlist_path
                        .take();
                    next_state.replace(State::Config(Box::new(config::State::new(ctx))));
                }
            }
        }
        if let Some(next_state) = next_state {
            self.state = next_state;
        }
    }
}
