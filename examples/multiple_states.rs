use std::{thread, time::Duration};

use log::{debug, info, LevelFilter};
use wolf_engine::{
    initialize_logging, Context, Engine, OptionalTransition, RenderResult, State, Transition,
};

pub fn main() {
    initialize_logging(LevelFilter::Debug);
    let state = MainState::new();

    Engine::new().run(Box::from(state));
}

pub struct MainState {
    number: u64,
}

impl MainState {
    pub fn new() -> Self {
        Self { number: 0 }
    }
}

impl State for MainState {
    fn update(&mut self, _context: &mut Context) -> OptionalTransition {
        if self.number == 10 {
            debug!("[MainState] All 10 messages displayed, quitting!");
            Some(Transition::Quit)
        } else {
            debug!("[MainState] Pushing new sub-state to the stack.");
            self.number += 1;
            let sub_state = SubState::new(format!("Hello, World! {}", self.number));
            Some(Transition::Push(Box::from(sub_state)))
        }
    }

    fn background_update(&mut self, _context: &mut Context) {
        debug!(
            "[MainState (Background)] Waiting for the message to be displayed by the sub-state."
        );
    }

    fn render(&mut self, _context: &mut Context) -> RenderResult {}
}

pub struct SubState {
    message: String,
    displayed_message: bool,
}

impl SubState {
    pub fn new(message: String) -> Self {
        Self {
            message,
            displayed_message: false,
        }
    }
}

impl State for SubState {
    fn update(&mut self, _context: &mut Context) -> OptionalTransition {
        if self.displayed_message {
            debug!("[SubState] The message was displayed.  Returning control to the main state.");
            Some(Transition::Pop)
        } else {
            debug!("[SubState] The message was not displayed.  Waiting for the next frame.");
            None
        }
    }

    fn render(&mut self, _context: &mut Context) -> RenderResult {
        info!("[SubState] {}", self.message);
        self.displayed_message = true;
        thread::sleep(Duration::from_millis(32));
    }
}
