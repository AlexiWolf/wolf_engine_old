use std::mem::replace;

use crate::plugins::CorePlugin;
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
/// # use wolf_engine::*;
/// #
/// # let my_game_state = EmptyState;
/// #
/// let engine = Engine::new();
/// ```
///
/// Using [Engine::default()] does the same thing:
///
/// ```
/// # use wolf_engine::*;
/// #
/// # let my_game_state = EmptyState;
/// #
/// let engine = Engine::default();
/// ```
///
/// If you don't want to use the default settings, the [EngineBuilder] and [Plugin]
/// system can be used to customize just about every aspect of the engine.
///
/// ```
/// # use wolf_engine::*;
/// #
/// // Add to the Context object here.
/// let engine = EngineBuilder::new()
///     // Customize the engine here.
///     .build()
///     .expect("Failed to build the Engine");
/// ```
///
/// You can refer to the [EngineBuilder] documentation for specifics on what it can do.
///
/// Running the engine is the same, no matter if you're using the default instance, or
/// a customized instance.  Just run [Engine::run()] and pass your game's starting [State]
/// to it.
///
/// ```
/// # use wolf_engine::*;
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
        self.state_stack.push(initial_state, &mut self.context);
        let (engine, core_function) = self.extract_core_function();
        (core_function)(engine);
        log_shutdown();
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

    /// Returns true if the engine is running.
    ///
    /// The engine is considered to be running when the following conditions are met:
    ///
    /// - There is at least one [State] on the [StateStack].
    pub fn is_running(&self) -> bool {
        self.state_stack.is_not_empty()
    }

    /// Triggers the start of a new frame.
    pub fn start_frame(&mut self) {
        puffin::GlobalProfiler::lock().new_frame()
    }

    /// Runs a complete update of all engine and game state.
    pub fn update(&mut self) {
        puffin::profile_scope!("update");
        self.scheduler
            .update(&mut self.context, &mut self.state_stack);
    }

    /// Renders the current frame.
    pub fn render(&mut self) {
        puffin::profile_scope!("render");
        self.scheduler
            .render(&mut self.context, &mut self.state_stack);
    }
}

impl Default for Engine {
    fn default() -> Self {
        EngineBuilder::new()
            .build()
            .expect("Failed to build the engine")
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
            .returning(|_| Some(Transition::Clean));
        state.expect_render().times(1..).returning(|_| ());
        state.expect_shutdown().times(1).returning(|_| ());

        wolf_engine.run(Box::from(state));
    }

    #[test]
    fn should_indicate_is_running_if_state_is_loaded() {
        let mut engine = Engine::default();
        let mut state = MockState::new();
        state.expect_setup().times(1).returning(|_| ());
        engine
            .state_stack
            .push(Box::from(state), &mut engine.context);

        assert!(
            engine.is_running(),
            "The Engine should indicate it is running."
        );
    }

    #[test]
    fn should_not_indicate_is_running_if_no_state_is_loaded() {
        let engine = Engine::default();

        assert!(
            !engine.is_running(),
            "The Engine should not indicate it is running."
        );
    }
}

/// Build and customize an instance of the [Engine].
///
/// The two main jobs of the engine builder is to load [Plugin]s and allow users to
/// customize the [Engine]'s settings.  The engine builder provides direct access to the
/// [Engine], and it's public types.
pub struct EngineBuilder {
    pub engine: Engine,
    plugin_loader: PluginLoader,
}

impl EngineBuilder {
    /// Creates a new engine builder with the default settings.
    pub fn new() -> Self {
        Self::default()
    }

    /// Consumes the engine builder and returns an [Engine] created from it.
    pub fn build(mut self) -> Result<Engine, String> {
        let plugin_loader = replace(&mut self.plugin_loader, PluginLoader::new());
        let engine_builder = plugin_loader.load_all(self);
        Ok(engine_builder.engine)
    }

    /// Set a custom [Scheduler] to be used.
    pub fn with_scheduler(mut self, scheduler: Box<dyn Scheduler>) -> Self {
        self.engine.scheduler = scheduler;
        self
    }

    /// Set a custom [CoreFunction] to be used.
    pub fn with_engine_core(mut self, engine_core: CoreFunction) -> Self {
        self.engine.core = engine_core;
        self
    }

    /// Add a [Plugin] to be loaded with the [Engine].
    pub fn with_plugin(mut self, plugin: Box<dyn Plugin>) -> Self {
        self.plugin_loader.add(plugin);
        self
    }

    /// Add a [Subcontext] to the [Engine].
    ///
    /// This method acts as a small wrapper around [Context::add()], except it won't fail
    /// if the [Subcontext] has already been added.
    pub fn with_subcontext<S: Subcontext>(mut self, subcontext: S) -> Self {
        self.engine.context.add(subcontext).unwrap();
        self
    }
}

impl Default for EngineBuilder {
    fn default() -> Self {
        Self {
            engine: Engine::empty(),
            plugin_loader: PluginLoader::new(),
        }
        .with_plugin(Box::from(CorePlugin))
        .with_engine_core(Box::from(run_while_has_active_state))
    }
}

#[cfg(test)]
mod engine_builder_tests {
    use std::sync::Mutex;

    use lazy_static::lazy_static;

    use crate::{
        contexts::{EventContext, SchedulerContext},
        event::Event,
    };

    use super::*;

    #[test]
    fn should_set_custom_scheduler() {
        let mut scheduler = MockScheduler::new();
        scheduler
            .expect_update()
            .times(1..)
            .returning(|context, state_stack| {
                state_stack.update(context);
            });
        scheduler.expect_render().times(..).return_const(());

        EngineBuilder::new()
            .with_scheduler(Box::from(scheduler))
            .build()
            .expect("Failed to build the engine")
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
            .build()
            .expect("Failed to build the engine");

        engine.run(Box::from(EmptyState));

        assert!(
            *HAS_RAN_CUSTOM_CORE.lock().unwrap(),
            "The custom engine core was not used"
        );
    }

    #[test]
    fn should_load_plugins() {
        let mut plugin = MockPlugin::new();
        plugin.expect_setup().times(1).returning(Ok);

        let _engine = EngineBuilder::new()
            .with_plugin(Box::from(plugin))
            .build()
            .expect("Failed to build the engine");
    }

    #[test]
    fn should_add_subcontexts_to_the_context_object() {
        let mut engine_builder = EngineBuilder::new();
        let subcontext = MockSubcontext::new();
        let starting_subcontexts = engine_builder.engine.context.len();

        engine_builder = engine_builder.with_subcontext(subcontext);

        let ending_subcontexts = engine_builder.engine.context.len();
        let subcontexts_added = ending_subcontexts - starting_subcontexts;
        assert_eq!(subcontexts_added, 1, "The subcontext was not added");
    }

    #[test]
    fn should_always_load_the_core_plugin() {
        let engine = EngineBuilder::new()
            .build()
            .expect("Failed to build the engine");

        let _event_context = engine
            .context
            .borrow::<EventContext<Event>>()
            .expect("failed to get EventContext<Event>");
        let _scheduler_context = engine
            .context
            .borrow::<SchedulerContext>()
            .expect("failed to get SchedulerContext");
    }
}
