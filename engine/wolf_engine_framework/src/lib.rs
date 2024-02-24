//! Provides a complete, batteries-included framework for Wolf Engine.
//!
//! The framework module provides a high-level framework for quickly and easily building games. It
//! makes several decisions, and handles a lot of the heavy lifting for you, so you can focus
//! more on building your game.  The framework includes a plugin system, and game state /
//! state-stack architecture.

pub mod scenes;
pub mod main_loop;

#[cfg(test)]
mod framework_runner_tests {
    use super::*;
    use wolf_engine_core::prelude::*;

    #[test]
    fn should_insert_main_loop_resource() {
        let engine = init()
            .with_main_loop(default_main_loop)
            .build()
            .unwrap();
    }
}
