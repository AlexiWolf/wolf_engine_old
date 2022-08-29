use std::{thread, time::Duration};

use log::*;
use wolf_engine::*;

pub fn main() {
    #[cfg(feature = "logging")]
    logging::initialize_logging(log::LevelFilter::Debug);

    let state = MyState::new("Hello, world!");

    Engine::new().run(Box::from(state));
}

pub struct MyState {
    message: String,
    updates: u64,
    frames: u64,
}

impl MyState {
    pub fn new(message: &str) -> Self {
        Self {
            message: message.to_string(),
            updates: 0,
            frames: 0,
        }
    }
}

impl State for MyState {
    fn update(&mut self, _context: &mut Context) -> OptionalTransition {
        if self.frames == 10 {
            info!("Goodbye!");
            Some(Transition::Clean)
        } else {
            self.updates += 1;
            debug!("Update: {}", self.updates);
            None
        }
    }

    fn render(&mut self, _context: &mut Context) {
        self.frames += 1;
        info!("{} {}", self.message, self.frames);
        thread::sleep(Duration::from_millis(32));
    }
}
