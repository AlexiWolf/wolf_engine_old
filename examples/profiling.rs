//! Run with `profiling` or `http_profiling` features enabled:
//!
//! - `cargo run --example profiling --features profiling`
//! or
//! - `cargo run --example profiling --features http_profiling`
//!
//! If you run with `http_profiling`, you can use
//! [Puffin Viewer](https://crates.io/crates/puffin_viewer) to view the profiler output:
//!
//! 1. `cargo install puffin_viewer`
//! 2. `puffin_viewer`

use std::thread::sleep;
use std::time::Duration;

use wolf_engine::*;
use wolf_engine::plugins::PuffinPlugin;
use wolf_engine::utils::{profile_function, profile_scope};
use log::*;

pub fn main() {
    #[cfg(feature = "logging")]
    logging::initialize_logging(LevelFilter::Debug);

    EngineBuilder::new()
        .with_plugin(Box::from(PuffinPlugin))
        .build()
        .run(Box::from(GameState));
}

pub struct GameState;

impl State for GameState {
    fn update(&mut self, _context: &mut Context) -> OptionalTransition {
        // Set a custom name for the profiler scope.
        profile_scope!("game_logic");
        None
    }

    fn render(&mut self, _context: &mut Context) -> RenderResult {
        // Allow Puffin to set profiler scope name based on the function name.
        profile_function!();
        sleep(Duration::from_millis(16)); // 60 fps.
    }
}
