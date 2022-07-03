use std::mem::replace;

use crate::*;
use crate::plugins::{CorePlugin, PuffinPlugin};
use crate::schedulers::FixedUpdateScheduler;
use crate::utils::EngineControls;

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
/// # Main Loops
///
/// The engine doesn't run the main loop on it's own.  Instead, it delegates the main loop
/// to an [MainLoop] implementation.  This helps to make the engine more modular, and
/// customizable.  A [MainLoop] can be used to change the specific way the engine runs
/// with ease, and is primarily used to integrate with 3rd party modules that insist
/// on being control of the main loop (such as Winit.)  See [MainLoop]'s documentation
/// for more details.
pub struct Engine {
    pub context: Context,
    pub scheduler: Box<dyn Scheduler>,
    pub state_stack: StateStack,
    main_loop: Box<dyn MainLoop>,
}

impl Engine {
    /// Creates and instance of the engine with the default settings.
    pub fn new() -> Self {
        Self::default()
    }

    /// Takes ownership over the engine and runs until the [MainLoop] exits.
    pub fn run(mut self, initial_state: Box<dyn State>) {
        log_startup_information();
        self.state_stack.push(initial_state, &mut self.context);
        let (mut engine, mut main_loop) = self.extract_core_function();
        engine = (main_loop).run(engine);
        engine.state_stack.clear(&mut engine.context);
        log_shutdown();
    }

    fn extract_core_function(mut self) -> (Engine, Box<dyn MainLoop>) {
        let mut engine = replace(&mut self, Self::empty());
        let engine_core = replace(&mut engine.main_loop, Box::from(EmptyMainLoop));
        (engine, engine_core)
    }

    fn empty() -> Self {
        Self {
            context: Context::default(),
            scheduler: Box::from(FixedUpdateScheduler::default()),
            state_stack: StateStack::new(),
            main_loop: Box::from(EmptyMainLoop),
        }
    }

    /// Returns true if the engine is running.
    ///
    /// The engine is considered to be running when the following conditions are met:
    ///
    /// - There is at least one [State] on the [StateStack].
    /// - [Engine::has_quit()] returns true.
    pub fn is_running(&self) -> bool {
        self.state_stack.is_not_empty() && !self.has_quit()
    }

    /// Triggers the start of a new frame.
    pub fn start_frame(&mut self) {
        profile_new_frame!();
    }

    /// Runs a complete update of all engine and game state.
    pub fn update(&mut self) {
        profile_scope!("update");
        self.scheduler
            .update(&mut self.context, &mut self.state_stack);
    }

    /// Renders the current frame.
    pub fn render(&mut self) {
        profile_scope!("render");
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
    use crate::contexts::EngineContext;
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

    #[test]
    fn should_have_engine_context() {
        let engine = Engine::default();

        let _engine_context = engine.context.borrow::<EngineContext>();
    }

    #[test]
    fn should_stop_running_when_quit_is_called() {
        let engine = Engine::default();
        let mut state = MockState::new();
        state.expect_setup().times(..).returning(|_| ());
        state.expect_update().times(1..).returning(|context| {
            context.quit();
            None
        });
        state.expect_render().times(..).returning(|_| ());
        state.expect_shutdown().times(1).returning(|_| ());

        engine.run(Box::from(state));
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
        match plugin_loader.load_all(self) {
            Ok(engine_builder) => Ok(engine_builder.engine),
            Err(error_message) => Err(error_message),
        }
    }

    /// Set a custom [Scheduler] to be used.
    pub fn with_scheduler(mut self, scheduler: Box<dyn Scheduler>) -> Self {
        self.engine.scheduler = scheduler;
        self
    }

    /// Set a custom [MainLoop] to be used.
    pub fn with_main_loop(mut self, engine_core: Box<dyn MainLoop>) -> Self {
        self.engine.main_loop = engine_core;
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

    fn load_default_plugins(mut self) -> Self {
        self = self.with_plugin(Box::from(CorePlugin));
        #[cfg(feature = "profiling")]
        { self = self.with_plugin(Box::from(PuffinPlugin)); }
        self
    }
}

impl Default for EngineBuilder {
    fn default() -> Self {
        Self {
            engine: Engine::empty(),
            plugin_loader: PluginLoader::new(),
        }
        .load_default_plugins()
        .with_main_loop(Box::from(DefaultMainLoop))
    }
}

#[cfg(test)]
mod engine_builder_tests {
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
    fn should_set_main_loop() {
        let mut main_loop = MockMainLoop::new();
        main_loop.expect_run().times(1).returning(|engine| engine);
        let engine = EngineBuilder::new()
            .with_main_loop(Box::from(main_loop))
            .build()
            .expect("Failed to build the engine");

        engine.run(Box::from(EmptyState));
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
    fn should_return_error_on_plugin_failure() {
        let mut plugin = MockPlugin::new();
        plugin
            .expect_setup()
            .once()
            .returning(|engine_builder| Err(("Test Error", engine_builder)));
        plugin.expect_name().once().returning(|| "Test Plugin");

        let result = EngineBuilder::new().with_plugin(Box::from(plugin)).build();

        assert!(result.is_err());
        assert_eq!(
            result.err().unwrap(),
            "Failed to load Test Plugin: Test Error"
        );
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
