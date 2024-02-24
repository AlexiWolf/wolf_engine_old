//! Provides a complete, batteries-included framework for Wolf Engine.
//!
//! The framework module provides a high-level framework for quickly and easily building games. It
//! makes several decisions, and handles a lot of the heavy lifting for you, so you can focus
//! more on building your game.  The framework includes a plugin system, and game state /
//! state-stack architecture.

pub mod scenes;
pub mod main_loop;

use main_loop::MainLoop;
use wolf_engine_core::engine_builder::EngineBuilder;

pub trait FrameworkBuilder {
    fn with_main_loop<T: MainLoop>(&mut self, main_loop: T) -> &mut Self;
}

impl<State> FrameworkBuilder for EngineBuilder<State> {
    fn with_main_loop<T: MainLoop>(&mut self, main_loop: T) -> &mut Self {
        self 
    }
}

#[cfg(test)]
mod framework_runner_tests {
    use crate::main_loop::{MockMainLoop, MainLoopResource};

    use super::*;
    use wolf_engine_core::prelude::*;

    #[test]
    fn should_insert_main_loop_resource() {
        let (_event_loop, context) = init()
            .with_main_loop(MockMainLoop::new())
            .build()
            .unwrap();

        assert!(context.resources().get::<MainLoopResource>().is_ok());
    }
}
