use std::mem::replace;

use crate::plugins::*;
use crate::schedulers::*;
use crate::utils::EngineControls;
use crate::*;

/// Provides the core functionality of Wolf Engine.
///
/// The `Engine` holds ownership over all major components such as the [Context], [StateStack], and
/// [schedulers].  Its main job is to take and run a set of [State] objects.  It also includes a
/// set of helper methods which provide Wolf Engine's default behaviors.
///
/// # Examples
///
/// If you just want to use the defaults, you can use [Engine::default()] or [Engine::default()].
///
/// ```
/// # use wolf_engine::*;
/// #
/// let new_engine = Engine::default();
/// // or
/// let default_engine = Engine::default();
/// ```
///
/// Otherwise, the [EngineBuilder] and [Plugin] system can be used to customize just about every
/// aspect of the `Engine`.
///
/// Create a new [EngineBuilder] by calling [Engine::builder()].
///
/// ```
/// # use wolf_engine::*;
/// #
/// // Add to the Context object here.
/// let engine = Engine::builder()
///     // Customize the engine here.
///     .build()
///     .expect("Failed to build the Engine");
/// ```
///
/// Refer to the [EngineBuilder] documentation for specifics on what it can do.
///
/// Running the engine is the same, no matter if you're using the default instance, or
/// a customized instance.  Call [Engine::run()] and pass your game's starting [State]
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
/// The `Engine` doesn't run on its own.  Instead, it delegates the run behavior to a [MainLoop]
/// implementation.  A [MainLoop] is used to customize the way the `Engine` runs, and are most
/// often used to integrate with other frameworks.  They may, however, be used to change the core
/// behavior of the `Engine` to better suit a projects needs.
///
/// By default, the `Engine` will use a [SimpleMainLoop].  [EngineBuilder::with_main_loop()], or a
/// [Plugin] can change which [MainLoop] is used.
#[derive(Debug)]
pub struct Engine {
    pub context: Context,
    pub state_stack: StateStack,
    pub stage_callbacks: StageCallbacks,
    update_scheduler: Box<dyn UpdateScheduler>,
    render_scheduler: Box<dyn RenderScheduler>,
    main_loop: Box<dyn MainLoop>,
}

impl Engine {
    /// Creates an instance of the [EngineBuilder].
    pub fn builder() -> EngineBuilder {
        EngineBuilder::new()
    }

    /// Takes ownership over the engine and runs until the [MainLoop] exits.
    pub fn run(mut self, initial_state: Box<dyn State>) {
        log_startup_information();
        self.state_stack.push(initial_state, &mut self.context);
        let (mut engine, mut main_loop) = self.replace_and_return_owned_main_loop();
        engine = (main_loop).run(engine);
        engine.state_stack.clear(&mut engine.context);
        log_shutdown();
    }

    fn replace_and_return_owned_main_loop(mut self) -> (Engine, Box<dyn MainLoop>) {
        let mut engine = replace(&mut self, Self::empty());
        let main_loop = replace(&mut engine.main_loop, Box::from(EmptyMainLoop));
        (engine, main_loop)
    }

    fn empty() -> Self {
        Self {
            context: Context::default(),
            state_stack: StateStack::new(),
            stage_callbacks: StageCallbacks::new(),
            update_scheduler: Box::from(FixedUpdateScheduler::default()),
            render_scheduler: Box::from(SimpleRenderScheduler),
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
    pub fn start_frame(&mut self) {}

    /// Runs a complete update of all engine and game state.
    pub fn update(&mut self) {
        self.update_scheduler
            .update(&mut self.context, &mut self.state_stack, &mut StageCallbacks::new());
    }

    /// Renders the current frame.
    pub fn render(&mut self) {
        self.render_scheduler
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
    use crate::{MockState, TransitionType};
    use ntest::timeout;

    use super::*;

    #[test]
    #[timeout(10)]
    fn should_run_the_state() {
        let wolf_engine = Engine::default();
        let mut state = MockState::new();
        state.expect_setup().times(1).returning(|_| ());
        state
            .expect_update()
            .times(1..)
            .returning(|_| Some(TransitionType::Clean));
        state.expect_render().times(..).returning(|_| ());
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
        let engine_context = engine.context.borrow::<EngineContext>();

        assert!(engine_context.is_some(), "There is no EngineContext loaded");
    }

    #[test]
    #[timeout(10)]
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
#[derive(Debug)]
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

    /// Set a custom [UpdateScheduler] to be used.
    pub fn with_update_scheduler(mut self, scheduler: Box<dyn UpdateScheduler>) -> Self {
        self.engine.update_scheduler = scheduler;
        self
    }

    /// Set a custom [RenderScheduler] to be used.
    pub fn with_render_scheduler(mut self, scheduler: Box<dyn RenderScheduler>) -> Self {
        self.engine.render_scheduler = scheduler;
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
        .with_main_loop(Box::from(SimpleMainLoop))
    }
}

#[cfg(test)]
mod engine_builder_tests {
    use ntest::timeout;

    use crate::contexts::SchedulerContext;

    use super::*;

    #[test]
    #[timeout(10)]
    fn should_set_custom_update_scheduler() {
        let mut scheduler = MockUpdateScheduler::new();
        scheduler
            .expect_update()
            .times(1..)
            .returning(|context, state_stack, _| {
                state_stack.update(context);
            });

        EngineBuilder::new()
            .with_update_scheduler(Box::from(scheduler))
            .build()
            .expect("Failed to build the engine")
            .run(Box::from(EmptyState));
    }

    #[test]
    #[timeout(10)]
    fn should_set_custom_render_scheduler() {
        let mut scheduler = MockRenderScheduler::new();
        scheduler.expect_render().times(1..).return_const(());

        EngineBuilder::new()
            .with_render_scheduler(Box::from(scheduler))
            .build()
            .expect("Failed to build the engine")
            .run(Box::from(EmptyState));
    }

    #[test]
    #[timeout(10)]
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

        let _scheduler_context = engine
            .context
            .borrow::<SchedulerContext>()
            .expect("failed to get SchedulerContext");
    }
}
