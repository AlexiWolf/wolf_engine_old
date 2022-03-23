use std::mem::replace;

use crate::contexts::EventContext;
use crate::event::Event;
use crate::schedulers::FixedUpdateScheduler;
use crate::*;

/// Provides the core functionality of the engine.
///
/// The engine is the core of, well, the engine.  It's primary job is to take and run a
/// set of game [State] objects.  The engine uses a [StateStack] to store all active
/// [State]s, and a [Scheduler] to control when things are run.
///
/// # Examples
///
/// If you just want to use the defaults, you can use [Engine::new()].
///
/// ```
/// # use wolf_engine::{Engine, EmptyState};
/// # let my_game_state = EmptyState;
/// #
/// let engine = Engine::new();
/// ```
///
/// Using [Engine::default()] does the same thing:
///
/// ```
/// # use wolf_engine::{Engine, EmptyState};
/// # let my_game_state = EmptyState;
/// #
/// let engine = Engine::default();
/// ```
///
/// If you don't want to use the default settings, the [EngineBuilder], and the [Context]
/// can be used to customize just about every aspect of the engine.
///
/// ```
/// # use wolf_engine::EngineBuilder;
/// #
/// // Add to the Context object here.
/// let engine = EngineBuilder::new()
///     // Customize the engine here.
///     .build();
/// ```
///
/// You can refer to the [EngineBuilder], and [Context] documentation for specifics on
/// each object can do.
///
/// Running the engine is the same, no matter if you're using the default instance, or
/// a customized instance.  Just run [Engine::run()] and pass your games starting [State]
/// to it.
///
/// ```
/// # use wolf_engine::{Engine, EmptyState};
/// #
/// # let engine = Engine::default();
/// # let my_game_state = EmptyState;
/// #
/// engine.run(Box::from(my_game_state));
/// ```
///
/// # Engine Cores
///
/// The engine doesn't run the main loop on it's own.  Instead, it delegates the main loop
/// to an [CoreFunction] function.  This helps to make the engine more modular, and
/// customizable.  An [CoreFunction] can be used to change the specific way the engine runs
/// with ease, and is primarily used to integrate with 3rd party modules that insist
/// on being control of the main loop (such as Winit.)  See [CoreFunction]'s documentation
/// for more details.
pub struct Engine {
    pub context: Context,
    pub scheduler: Box<dyn Scheduler>,
    pub state_stack: StateStack,
    core: CoreFunction,
}

impl Engine {
    /// Creates and instance of the engine with the default settings.
    pub fn new() -> Self {
        Self::default()
    }

    /// Takes ownership over the engine and runs until the [CoreFunction] exits.
    pub fn run(mut self, initial_state: Box<dyn State>) {
        log_startup_information();
        self.add_required_subcontexts();
        self.state_stack.push(initial_state, &mut self.context);
        let (engine, core_function) = self.extract_core_function();
        (core_function)(engine);
    }

    fn add_required_subcontexts(&mut self) {
        self.context.add(EventContext::<Event>::default()).unwrap();
    }

    fn extract_core_function(mut self) -> (Engine, Box<dyn Fn(Engine)>) {
        let mut engine = replace(&mut self, Self::empty());
        let engine_core = replace(&mut engine.core, Box::from(|_| {}));
        (engine, engine_core)
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
        EngineBuilder::new().build()
    }
}

/// Build and customize an instance of the [Engine].
pub struct EngineBuilder {
    context: Context,
    scheduler: Box<dyn Scheduler>,
    core: CoreFunction,
}

impl EngineBuilder {
    /// Creates a new engine builder with the default settings.
    pub fn new() -> Self {
        Self::default()
    }

    /// Consumes the engine builder and returns an [Engine] created from it.
    pub fn build(self) -> Engine {
        Engine {
            context: self.context,
            scheduler: self.scheduler,
            state_stack: StateStack::new(),
            core: self.core,
        }
    }

    /// Set a custom [Scheduler] to be used.
    pub fn with_scheduler(mut self, scheduler: Box<dyn Scheduler>) -> Self {
        self.scheduler = scheduler;
        self
    }

    /// Set a custom [CoreFunction] to be used.
    pub fn with_engine_core(mut self, engine_core: CoreFunction) -> Self {
        self.core = engine_core;
        self
    }
}

impl Default for EngineBuilder {
    fn default() -> Self {
        Self {
            context: Context::empty(),
            scheduler: Box::from(FixedUpdateScheduler::default()),
            core: Box::from(run_while_has_active_state),
        }
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
        state.expect_setup().times(..).returning(|_| ());
        state
            .expect_update()
            .times(1..)
            .returning(|_| Some(Transition::Quit));
        state.expect_render().times(1..).returning(|_| ());
        state.expect_shutdown().times(1).returning(|_| ());

        wolf_engine.run(Box::from(state));
    }
}

#[cfg(test)]
mod engine_builder_tests {
    use std::sync::Mutex;

    use lazy_static::lazy_static;

    use super::*;

    #[test]
    fn should_allow_custom_states() {
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
            .build()
            .run(Box::from(EmptyState));
    }

    #[test]
    fn should_set_engine_core() {
        lazy_static! {
            static ref HAS_RAN_CUSTOM_CORE: Mutex<bool> = Mutex::from(false);
        }
        let engine = EngineBuilder::new()
            .with_engine_core(Box::from(|_| {
                *HAS_RAN_CUSTOM_CORE.lock().unwrap() = true;
            }))
            .build();

        engine.run(Box::from(EmptyState));

        assert!(
            *HAS_RAN_CUSTOM_CORE.lock().unwrap(),
            "The custom engine core was not used"
        );
    }

    #[test]
    fn should_add_event_context_at_startup() {
        Engine::new().run(Box::from(AddEventContextTestState));
    }

    struct AddEventContextTestState;

    impl State for AddEventContextTestState {
        fn update(&mut self, context: &mut Context) -> OptionalTransition {
            context
                .get::<EventContext<Event>>()
                .expect("no EventContext");
            Some(Transition::Quit)
        }

        fn render(&mut self, _context: &mut Context) -> RenderResult {}
    }
}
