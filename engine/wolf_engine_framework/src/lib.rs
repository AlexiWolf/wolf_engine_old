//! Provides a complete, batteries-included framework for Wolf Engine.
//!
//! The framework module provides a high-level framework for quickly and easily building games. It
//! makes several decisions, and handles a lot of the heavy lifting for you, so you can focus
//! more on building your game.  The framework includes a plugin system, and game state /
//! state-stack architecture.

pub mod scenes;
pub mod main_loop;

use main_loop::{MainLoop, MainLoopResource};
use wolf_engine_core::{engine_builder::EngineBuilder, Engine};

pub trait FrameworkBuilder {
    fn with_main_loop<T: MainLoop + 'static>(&mut self, main_loop: T) -> &mut Self;
}

impl<State> FrameworkBuilder for EngineBuilder<State> {
    fn with_main_loop<T: MainLoop + 'static>(&mut self, main_loop: T) -> &mut Self {
        self.with_resource(MainLoopResource::new(main_loop))
    }
}

/// Runs the [`Engine`].
pub fn run(engine: Engine) {
    let (event_loop, mut context) = engine;
    let mut main_loop = context.resources_mut()
        .remove::<MainLoopResource>()
        .unwrap_or(MainLoopResource::new(default_main_loop))
        .extract();
    main_loop.run((event_loop, context));
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

    #[test]
    fn should_use_main_loop() {
        let mut main_loop = MockMainLoop::new();
        main_loop.expect_run().once().return_const(());

        let engine = init()
            .with_main_loop(main_loop)
            .build()
            .unwrap();

        run(engine);
    }

    #[test]
    fn should_use_default_main_loop() {
        let engine = init()
            .build()
            .unwrap();
        run(engine);
    }
}

pub(crate) fn default_main_loop(engine: Engine) {}

#[cfg(test)]
mod default_main_loop_tests {
    use super::*;
    use wolf_engine_core::prelude::*;
    use ntest::timeout;

    #[test]
    #[timeout(100)]
    fn should_exit_on_quit() {
        let engine = init().build().unwrap();
        run(engine);
    }
}

