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
/// change the [Engine]'s core behavior wihout needing to rewrite its code.  Using an 
/// engine core, you could, for example:
///
/// - Change the behavior of the main loop to better suit your game's needs.
/// - Integrate with 3rd party frameworks (such as Winit, Call Loop, or Tokio), and allow
///   them to control the main loop.
/// - Extend existing engine cores with useful debugging features.
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
    context: Context,
    scheduler: Box<dyn Scheduler>,
    state_stack: StateStack,
}

impl Engine {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn run(mut self, initial_state: Box<dyn State>) {
        self.state_stack.push(initial_state);
        while !self.state_stack.is_empty() {
            self.scheduler
                .update(&mut self.context, &mut self.state_stack);
            self.scheduler
                .render(&mut self.context, &mut self.state_stack);
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
        }
    }

    pub fn with_scheduler(mut self, scheduler: Box<dyn Scheduler>) -> Self {
        self.scheduler = scheduler;
        self
    }
}

impl Default for EngineBuilder {
    fn default() -> Self {
        Self {
            scheduler: Box::from(FixedUpdateScheduler::default()),
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
}
