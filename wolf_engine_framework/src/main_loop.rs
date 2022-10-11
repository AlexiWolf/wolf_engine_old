//! Provides interchangeable main-loops for the [Engine].

use std::fmt::Debug;

use crate::*;

#[cfg(test)]
use mockall::automock;

/// Provides the main-loop for the [Engine].
///
/// The `MainLoop` loop is responsible for actually running the [Engine], and can directly control
/// the specifics of how the [Engine] works.  When [Engine::run()] is called, and after all start-up
/// code has completed, [MainLoop::run()] is called and given ownership over the [Engine].
///
/// The main reason for separating the `MainLoop` from the [Engine] to make it easy to change the
/// [Engine]'s behavior, in a way that is transparent to the user, without requiring the [Engine]'s
/// code to be re-written.  The most common reason to change the `MainLoop` is to integrate Wolf
/// Engine into other frameworks which take control over the program's main loop (Winit's
/// `EventLoop` for example.)
///
/// **Note:** Keep in mind `MainLoop` implementations are **not required to preserve Wolf Engine's
/// default behaviors**, and may make changes to suit their needs.  You should refer to the
/// implementation's documentation for specific details.
///
/// By default, Wolf Engine will select [SimpleMainLoop].
///
/// # Examples
///
/// Using a different `MainLoop`:
///
/// A custom `MainLoop` can be set using
/// [EngineBuilder::with_main_loop()](crate::EngineBuilder::with_main_loop()) at startup.
///
/// ```
/// # use wolf_engine_framework::*;
/// #
/// # let custom_engine_core = EmptyMainLoop;
/// #
/// let engine = EngineBuilder::new()
///     .with_main_loop(Box::from(custom_engine_core))
///     .build();
/// ```
/// Implementing a custom `MainLoop`:
///
/// If you want to preserve the default behaviors, the [Engine] provides a number of helper methods
/// to make it easy:
///
/// - [Engine::is_running()]
/// - [Engine::start_frame()]
/// - [Engine::update()]
/// - [Engine::render()]
///
/// Then you implement the `MainLoop` trait for your `MainLoop`.
///
/// ```
/// # use wolf_engine_framework::*;
/// #
/// #[derive(Debug)]
/// pub struct CustomMainLoop;
///
/// impl MainLoop for CustomMainLoop {
///     fn run(&mut self, mut engine: Engine) -> Engine {
///         while engine.is_running() {
///             engine.start_frame();
///             engine.update();
///             engine.render();
/// #           break
///         }
///         engine
///     }
/// }
/// ```
///
/// If you want to implement your own behaviors, the process is largely the same except you
/// will need to work with the [Engine]'s components directly.
#[cfg_attr(test, automock)]
pub trait MainLoop: Debug {
    fn run(&mut self, engine: Engine) -> Engine;
}

/// Provides the default main loop behavior.
///
/// This is a minimal reference implementation of [MainLoop] which uses a `while` loop to run the
/// [Engine].  The main loop will exit when [Engine::is_running()] returns false.
#[derive(Debug)]
pub struct SimpleMainLoop;

impl MainLoop for SimpleMainLoop {
    fn run(&mut self, mut engine: Engine) -> Engine {
        while engine.is_running() {
            engine.start_frame();
            engine.update();
            engine.render();
        }
        engine
    }
}

#[doc(hidden)]
#[derive(Debug)]
pub struct EmptyMainLoop;

impl MainLoop for EmptyMainLoop {
    fn run(&mut self, engine: Engine) -> Engine {
        engine
    }
}
