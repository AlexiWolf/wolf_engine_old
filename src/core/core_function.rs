//! Provides swapable [EngineCore] functions.

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
///     loop {
///         engine.scheduler
///             .update(&mut engine.context, &mut engine.state_stack);
///         engine.scheduler
///             .render(&mut engine.context, &mut engine.state_stack);
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
/// # let context = Context::default();
/// #
/// let engine = EngineBuilder::new()
///     .with_engine_core(Box::from(custom_engine_core))
///     .build(context);
/// ```
pub type EngineCore = Box<dyn Fn(Engine)>;

/// Run the [Engine] until the [StateStack](crate::StateStack) is empty.
///
/// This is a simple [EngineCore] that runs the engine in a loop.  It will run the
/// [Engine]'s [StateStack](crate::StateStack) using the active
/// [Scheduler](crate::Scheduler).  The loop will continue to run until the
/// [StateStack](crate::StateStack)is empty, then it will exit.
pub fn run_engine(mut engine: Engine) {
    while engine.state_stack.is_not_empty() {
        engine
            .scheduler
            .update(&mut engine.context, &mut engine.state_stack);
        engine
            .scheduler
            .render(&mut engine.context, &mut engine.state_stack);
    }
}
