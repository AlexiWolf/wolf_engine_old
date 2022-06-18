//! Provides swapable [MainLoop] functions.

use crate::Engine;

#[cfg(test)]
use mockall::automock;

/// Defines which functions can be used as an [Engine] core.
///
/// Main loops take ownership over the running [Engine], and directly implement the
/// main loop for the [Engine].  When [Engine::run()] is called, the main loop behavior
/// is delegated to the main loop implementation provided at startup.
///
/// The main reason for separating the core main loop from the [Engine] to make it easy
/// to change the [Engine]'s core behavior without needing to rewrite its code.  Using a
/// a main loop, you could, for example:
///
/// - Change the behavior of the main loop to better suit your game's needs.
/// - Integrate with 3rd party frameworks (such as Winit, SDL, or Tokio), and allow
///   them to control the main loop.
///
///
/// # Examples
///
/// To use the default main loop, you don't need to do anything special.  The engine will 
/// automatically select [DefaultMainLoop] unless a different one is selected either by you, or, 
/// more commonly, by a [Plugin](crate::Plugin).
/// 
/// If you wish to override the engine's main loop behavior, you can simply implement this trait
/// on a struct.
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
/// To use a custom engine core, the core is [Box]ed, then passed to
/// [EngineBuilder::with_engine_core()](crate::EngineBuilder) method at startup.
///
/// ```
/// # use wolf_engine::*;
/// #
/// # let custom_engine_core = EmptyMainLoop;
/// #
/// let engine = EngineBuilder::new()
///     .with_engine_core(Box::from(custom_engine_core))
///     .build();
/// ```
#[cfg_attr(test, automock)]
pub trait MainLoop {
    fn run(&mut self, engine: Engine) -> Engine;
}

/// Provides the default main loop behavior.
///
/// This is a minimal reference implementation of [MainLoop].  The main loop will exit when 
/// [Engine::is_running()] returns false.
pub struct DefaultMainLoop;

impl MainLoop for DefaultMainLoop {
    fn run(&mut self, mut engine: Engine) -> Engine {
        while engine.is_running() {
            engine.start_frame();
            puffin::profile_scope!("frame");
            engine.update();
            engine.render();
        }
        log::debug!("The Engine has quit, shutting down now.");
        engine
    }
}

pub(crate) struct EmptyMainLoop;

impl MainLoop for EmptyMainLoop {
    fn run(&mut self, engine: Engine) -> Engine {
        engine 
    }
}
