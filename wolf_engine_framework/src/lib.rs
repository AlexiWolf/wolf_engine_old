//! Provides a complete, batteries-included framework for Wolf Engine.
//!
//! The framework module provides a high-level framework for quickly and easily building games. It
//! makes several decisions, and handles a lot of the heavy lifting for you, so you can focus
//! more on building your game.  The framework includes a plugin system, and game state /
//! state-stack architecture.

mod framework_builder;
pub use framework_builder::*;

pub mod plugins;

use wolf_engine_core::{events::UserEvent, Engine};

/// Initializes Wolf Engine using the [`FrameworkBuilder`].
pub fn init<E: UserEvent>() -> FrameworkBuilder<E> {
    let mut builder = FrameworkBuilder::<E>::new();
    builder.with_resource(MainLoopResource::<E> {
        inner: Box::from(|_engine| {}),
    });
    builder
}

#[cfg(test)]
mod framework_init_tests {
    pub struct TestResourceA;
    pub struct TestResourceB;

    #[test]
    fn should_add_resources() {
        let (_event_loop, context) = crate::init::<()>()
            .with_resource(TestResourceA)
            .with_resource(TestResourceB)
            .build()
            .unwrap();
        assert!(
            context.resources().get::<TestResourceA>().is_some(),
            "Resource insertion failed"
        );
        assert!(
            context.resources().get::<TestResourceB>().is_some(),
            "Resource insertion failed"
        );
    }
}

pub(crate) struct MainLoopResource<E: UserEvent> {
    inner: Box<dyn MainLoop<E>>,
}

impl<E: UserEvent> MainLoopResource<E> {
    pub fn extract(self) -> Box<dyn MainLoop<E>> {
        self.inner 
    }
}

pub trait MainLoop<E: UserEvent> {
    fn run(&mut self, engine: Engine<E>);
}

impl<E: UserEvent, T> MainLoop<E> for T where T: FnMut(Engine<E>) {
    fn run(&mut self, engine: Engine<E>) {
        (self)(engine)
    }
}

#[cfg(test)]
mod framework_runner_test {
    use super::*;

    #[test]
    fn should_add_main_loop_resource() {
        let (_event_loop, context) = crate::init::<()>()
            .build()
            .unwrap();

        assert!(
            context.resources().get::<MainLoopResource<()>>().is_some(),
            "Main loop resource was not inserted"
        );
    }

    #[test]
    fn should_add_custom_main_loop() {
        let mut has_run = false;
        let (event_loop, mut context) = crate::init::<()>()
            .with_main_loop(|engine| {
                has_run = true;
            })
            .build()
            .unwrap();

        let mut main_loop = context.resources_mut()
            .remove::<MainLoopResource<()>>()
            .expect("No MainLoopResource")
            .extract();

        main_loop.run((event_loop, context));

        assert!(has_run, "The main loop did not run, or was not correct");
    }
}
