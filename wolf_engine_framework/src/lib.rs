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
        inner: Box::from(main_loop),
    });
    builder
}

/// Runs the [`Engine`].
///
/// # Panics
///
/// This function expects you to use the Framework's [wolf_enigne::framework::init()](init)
/// function to create the [`Engine`], otherwise, this function will panic.
pub fn run<E: UserEvent>(engine: Engine<E>) {
    let (event_loop, mut context) = engine;

    let mut main_loop = context.resources_mut()
        .remove::<MainLoopResource<E>>()
        .expect(
            "No main loop.  Make sure you used `wolf_engine::framework::init()` to set up the Engine")
        .extract();

    main_loop.run((event_loop, context));
}

/// The default [`MainLoop`] implementation.
pub(crate) fn main_loop<E: UserEvent>(engine: Engine<E>) {}

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

/// Provides a wrapper around some [`MainLoop`] implementation, making it possible to access it as
/// a [`Resource`] at run-time.
pub(crate) struct MainLoopResource<E: UserEvent> {
    inner: Box<dyn MainLoop<E>>,
}

impl<E: UserEvent> MainLoopResource<E> {
    /// Sets the inner [`MainLoop`].
    pub fn set_main_loop(&mut self, main_loop: Box<dyn MainLoop<E>>) {
        self.inner = main_loop;
    }
    
    /// Consumes the resource, and returns a pointer to underlying [`MainLoop`].
    pub fn extract(self) -> Box<dyn MainLoop<E>> {
        self.inner
    }
}

/// An implementation of the engine's main-loop.
#[cfg_attr(test, mockall::automock)]
pub trait MainLoop<E: UserEvent> {
    /// Runs the main-loop until the engine quits.
    fn run(&mut self, engine: Engine<E>);
}

impl<E: UserEvent, T> MainLoop<E> for T
where
    T: FnMut(Engine<E>),
{
    fn run(&mut self, engine: Engine<E>) {
        (self)(engine)
    }
}

#[cfg(test)]
mod framework_runner_test {
    use super::*;

    #[test]
    fn should_add_main_loop_resource() {
        let (_event_loop, context) = crate::init::<()>().build().unwrap();

        assert!(
            context.resources().get::<MainLoopResource<()>>().is_some(),
            "Main loop resource was not inserted"
        );
    }

    #[test]
    fn should_add_custom_main_loop() {
        let mut mock_main_loop = MockMainLoop::new();
        mock_main_loop.expect_run().once().return_const(());

        let engine = crate::init::<()>()
            .with_main_loop(mock_main_loop)
            .build()
            .unwrap();

        crate::run(engine);
    }

    #[test]
    #[should_panic]
    fn should_panic_without_main_loop() {
        let engine = wolf_engine_core::init::<()>().build();

        crate::run(engine);
    }
}
