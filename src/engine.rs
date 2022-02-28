use std::mem::replace;

use crate::{
    scheduler::{FixedUpdateScheduler, Scheduler},
    Context, State, StateStack,
};

/// Provides the core behavior of the [Engine].
///
/// Engine cores take ownership over the running [Engine], and directly provide core 
/// behavior at run-time.  In most cases, running some sort of main loop.
///
/// The main reason for separating the main loop, from the [Engine] to make it easy to
/// change the [Engine]'s core behavior without needing to rewrite its code.  Using an 
/// engine core, you could, for example:
///
/// - Change the behavior of the main loop to better suit your game's needs.
/// - Integrate with 3rd party frameworks (such as Winit, Call Loop, or Tokio), and allow
///   them to control the main loop.
/// - Extend existing engine cores with useful debugging features.
///
/// # Examples
///
/// ## Implementing an Engine Core
/// 
/// Any function that takes an [Engine] as an argument, and that does not have a return 
/// type can be used as an engine core.
///
/// ```
/// # use wolf_engine::{Engine, EngineCore};
/// #
/// pub fn custom_engine_core(engine: Engine) {
///     loop {
/// #       break
///     }
/// }
/// ```
///
/// ## Using a Custom Engine Core
///
/// To use a custom engine core, the core can be passed to the 
/// [EngineBuilder::with_engine_core()] method at startup.
///
/// ```
/// # use wolf_engine::{Context, EngineBuilder, run_engine};
/// # 
/// # let custom_engine_core = run_engine;
/// # let context = Context::default();
/// #
/// let engine = EngineBuilder::new()
///     .with_engine_core(Box::from(custom_engine_core))
///     .build(context);
/// ```
pub type EngineCore = Box<dyn Fn(Engine)>;

/// Provides the core functionality of the engine.
///
/// The main job of the engine is to run the main loop. The engine takes an initial
/// [State] object and pushes that onto it's internal [StateStack].  It will then run your
/// game through the [StateStack] utilizing a [Scheduler] for timing control.
///
/// If you just want to use the defaults, you can use [Engine::new()].  Of course, you'll
/// need to pass it your game's [State]:
///
/// ```
/// # use wolf_engine::{Engine, EmptyState};
/// # let my_game_state = EmptyState;
///
/// Engine::new()
///     .run(Box::from(my_game_state));
/// ```
///
/// Using [Engine::default()] does the same thing:
///
/// ```
/// # use wolf_engine::{Engine, EmptyState};
/// # let my_game_state = EmptyState;
/// #
/// Engine::default()
///     .run(Box::from(my_game_state));
/// ```
pub struct Engine {
    pub context: Context,
    pub scheduler: Box<dyn Scheduler>,
    pub state_stack: StateStack,
    core: EngineCore,
}

impl Engine {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn run(mut self, initial_state: Box<dyn State>) {
        self.state_stack.push(initial_state);

        let mut engine = replace(&mut self, Self::empty());
        let engine_core = replace(&mut engine.core, Box::from(|_| {}));

        (engine_core)(engine);
    }

    fn empty() -> Self {
        Self {
            context: Context::default(), 
            scheduler: Box::from(FixedUpdateScheduler::default()), 
            state_stack: StateStack::new(), 
            core: Box::from(|_| {}), 
        }
    }
}

impl Default for Engine {
    fn default() -> Self {
        let context = Context::default();
        EngineBuilder::new().build(context)
    }
}

/// Build and customize an instance of the [Engine].
pub struct EngineBuilder {
    scheduler: Box<dyn Scheduler>,
    core: EngineCore,
}

impl EngineBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(self, context: Context) -> Engine {
        Engine {
            context,
            scheduler: self.scheduler,
            state_stack: StateStack::new(),
            core: self.core,
        }
    }

    pub fn with_scheduler(mut self, scheduler: Box<dyn Scheduler>) -> Self {
        self.scheduler = scheduler;
        self
    }

    pub fn with_engine_core(mut self, engine_core: EngineCore) -> Self {
        self.core = engine_core;
        self
    }
}

impl Default for EngineBuilder {
    fn default() -> Self {
        Self {
            scheduler: Box::from(FixedUpdateScheduler::default()),
            core: Box::from(run_engine),
        }
    }
}

/// Run the [Engine] until the [StateStack] is empty. 
/// 
/// This is a simple [EngineCore] that runs the engine in a loop.  It will run the 
/// [Engine]'s [StateStack] using the active [Scheduler].  The loop will continue to run 
/// until the [StateStack] is empty, then it will exit.
pub fn run_engine(mut engine: Engine) {
   while !engine.state_stack.is_empty() {
       engine.scheduler
           .update(&mut engine.context, &mut engine.state_stack);
       engine.scheduler
           .render(&mut engine.context, &mut engine.state_stack);
   }
}

#[cfg(test)]
mod wolf_engine_tests {
    use crate::{MockState, Transition};

    use super::*;

    #[test]
    fn should_run_the_state() {
        let wolf_engine = Engine::default();
        let mut state = MockState::new();
        state
            .expect_update()
            .times(1..)
            .returning(|_| Some(Transition::Quit));
        state.expect_render().times(1..).returning(|_| ());

        wolf_engine.run(Box::from(state));
    }
}

#[cfg(test)]
mod engine_builder_tests {
    use std::sync::Mutex;

    use lazy_static::lazy_static;

    use super::*;
    use crate::{scheduler::MockScheduler, EmptyState};

    #[test]
    fn should_allow_custom_states() {
        let context = Context::default();
        let mut scheduler = MockScheduler::new();
        scheduler
            .expect_update()
            .times(1)
            .returning(|context, state_stack| {
                state_stack.update(context);
            });
        scheduler.expect_render().times(..).return_const(());

        EngineBuilder::new()
            .with_scheduler(Box::from(scheduler))
            .build(context)
            .run(Box::from(EmptyState));
    }

    #[test]
    fn should_set_engine_core() {
        lazy_static! { static ref HAS_RAN_CUSTOM_CORE: Mutex<bool> = Mutex::from(false); }
        let context = Context::default();
        let engine = EngineBuilder::new()
            .with_engine_core(Box::from(|_| { *HAS_RAN_CUSTOM_CORE.lock().unwrap() = true; }))
            .build(context);

        engine.run(Box::from(EmptyState));

        assert!(*HAS_RAN_CUSTOM_CORE.lock().unwrap(), "The custom engine core was not used");
    }
}
