use std::{thread, time::Duration};

use log::*;
use wolf_engine::*;

pub fn main() {
    #[cfg(feature = "logging")]
    logging::initialize_logging(LevelFilter::Debug);

    let state = MainState::new();

    Engine::default().run(Box::from(state));
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
    fn update(&mut self, _context: &mut Context) -> Transition {
        if self.number == 10 {
            debug!("[MainState] All 10 messages displayed, quitting!");
            Some(TransitionType::Clean)
        } else {
            debug!("[MainState] Pushing new sub-state to the stack.");
            self.number += 1;
            let sub_state = SubState::new(format!("Hello, World! {}", self.number));
            Some(TransitionType::Push(Box::from(sub_state)))
        }
    }

    fn background_update(&mut self, _context: &mut Context) {
        debug!(
            "[MainState (Background)] Waiting for the message to be displayed by the sub-state."
        );
    }

    fn render(&mut self, _context: &mut Context) {}
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
    fn update(&mut self, _context: &mut Context) -> Transition {
        if self.displayed_message {
            debug!("[SubState] The message was displayed.  Returning control to the main state.");
            Some(TransitionType::Pop)
        } else {
            debug!("[SubState] The message was not displayed.  Waiting for the next frame.");
            None
        }
    }

    fn render(&mut self, _context: &mut Context) {
        info!("[SubState] {}", self.message);
        self.displayed_message = true;
        thread::sleep(Duration::from_millis(32));
    }
}
