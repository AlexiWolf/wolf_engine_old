//! Provides swappable [MainLoop] functions.

use crate::*;

#[cfg(test)]
use mockall::automock;

/// Provides a dynamic main loop for the [Engine].
///
/// Main loops take ownership over the running [Engine] after it is run.   When [Engine::run()] is
/// called, the main loop behavior is delegated to the main loop implementation provided at
/// startup.  By default, the [Engine] will use the [DefaultMainLoop].
///
/// The main reason for separating the main loop from the [Engine] to make it easy to change the
/// [Engine]'s behavior without needing to rewrite its code.  Using a main loop, you could, for
/// example:
///
/// - Change the behavior of the main loop to better suit your game's needs.
/// - Integrate with 3rd party frameworks (such as Winit, or SDL).
///
/// # Examples
///
/// To override the engine's main loop behavior, start by implementing this trait for a struct.
///
/// ```
/// # use wolf_engine::*;
/// #
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
/// Then set the main loop with [EngineBuilder::with_engine_core()](crate::EngineBuilder) at startup.
///
/// ```
/// # use wolf_engine::*;
/// #
/// # let custom_engine_core = EmptyMainLoop;
/// #
/// let engine = EngineBuilder::new()
///     .with_main_loop(Box::from(custom_engine_core))
///     .build();
/// ```
#[cfg_attr(test, automock)]
pub trait MainLoop {
    fn run(&mut self, engine: Engine) -> Engine;
}

/// Provides the default main loop behavior.
///
/// This is a minimal reference implementation of [MainLoop] which uses a `while` loop to run the
/// [Engine].  The main loop will exit when [Engine::is_running()] returns false.
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
pub struct EmptyMainLoop;

impl MainLoop for EmptyMainLoop {
    fn run(&mut self, engine: Engine) -> Engine {
        engine
    }
}
