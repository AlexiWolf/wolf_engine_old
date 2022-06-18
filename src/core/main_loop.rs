//! Provides swapable [CoreFunction] functions.

use crate::Engine;

/// Defines which functions can be used as an [Engine] core.
///
/// Core functions take ownership over the running [Engine], and directly implement the
/// main-loop for the [Engine].  When [Engine::run()] is called, the main-loop behavior
/// is delegated to the core function provided at startup.
///
/// The main reason for separating the core main-loop from the [Engine] to make it easy
/// to change the [Engine]'s core behavior without needing to rewrite its code.  Using a
/// core function, you could, for example:
///
/// - Change the behavior of the main loop to better suit your game's needs.
/// - Integrate with 3rd party frameworks (such as Winit, Calloop, or Tokio), and allow
///   them to control the main loop.
/// - Extend existing engine cores with useful debugging features.
///
/// [run_engine()] is the default core function.
///
/// # Examples
///
/// Any function that takes an [Engine] as an argument and does not have a return type can
/// be used as an engine core.  To create a custom engine core, you just implement it as
/// a function:
///
/// ```
/// # use wolf_engine::*;
/// #
/// pub fn custom_engine_core(mut engine: Engine) {
///     while engine.is_running() {
///         engine.start_frame();
///         engine.update();
///         engine.render();
/// #       break
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
/// # let custom_engine_core = run_engine;
/// #
/// let engine = EngineBuilder::new()
///     .with_engine_core(Box::from(custom_engine_core))
///     .build();
/// ```
pub type CoreFunction = Box<dyn Fn(Engine) -> Engine>;

/// Runs the [Engine] until it has quit.  
///
/// This is the default [CoreFunction] implementation.
///
/// The main loop will exit when [Engine::is_running()] returns false.
pub fn run_engine(mut engine: Engine) -> Engine {
    while engine.is_running() {
        engine.start_frame();
        puffin::profile_scope!("frame");
        engine.update();
        engine.render();
    }
    log::debug!("The Engine has quit, shutting down now.");
    engine
}
