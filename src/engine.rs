#[cfg(feature = "window")]
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    platform::run_return::EventLoopExtRunReturn,
};

use crate::{
    scheduler::{FixedUpdateScheduler, Scheduler},
    Context, State, StateStack,
};

/// Provides the core functionality of the engine.
///
/// The main job of the engine is to run the main loop. The engine takes an initial
/// [State] object and pushes that onto it's internal [StateStack].  It will then run your
/// game through the [StateStack] utilizing a [Scheduler] for timing control.
///
/// If you just want to use the defaults, you can use `Engine::new()`.  Of course, you'll
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
/// Using Engine::default() does the same thing:
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

#[cfg(feature = "window")]
impl Engine {
    pub fn run_with_event_loop(mut self, initial_state: Box<dyn State>, event_loop: EventLoop<()>) {
        self.state_stack.push(initial_state);
        self.run_event_loop(event_loop);
    }

    fn run_event_loop(&mut self, mut event_loop: EventLoop<()>) {
        event_loop.run_return(|event, _window, control_flow| {
            match event {
                Event::MainEventsCleared => {
                    self.scheduler
                        .update(&mut self.context, &mut self.state_stack);
                }
                Event::RedrawRequested(_) => {
                    self.scheduler
                        .render(&mut self.context, &mut self.state_stack);
                }
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    ..
                } => {
                    self.state_stack.clear();
                }
                _ => (),
            }
            if self.state_stack.is_empty() {
                *control_flow = ControlFlow::Exit;
            }
        });
    }
}

impl Default for Engine {
    fn default() -> Self {
        let context = Context::default();
        EngineBuilder::new()
            .build(context)
    }
}

/// Build and customize an instance of the [Engine].
pub struct EngineBuilder {
    scheduler: Box<dyn Scheduler> 
}

impl EngineBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(self, context: Context) -> Engine {
        Engine { 
            context, 
            scheduler: self.scheduler, 
            state_stack: StateStack::new()
        } 
    }

    pub fn with_scheduler(mut self, scheduler: Box<dyn Scheduler>) -> Self {
        self.scheduler = scheduler;
        self 
    }
}

impl Default for EngineBuilder {
    fn default() -> Self {
        Self { scheduler: Box::from(FixedUpdateScheduler::default()) }
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
        scheduler.expect_update()
            .times(1)
            .returning(|context, state_stack| { state_stack.update(context); });
        scheduler.expect_render().times(..).return_const(());

        EngineBuilder::new()
            .with_scheduler(Box::from(scheduler))
            .build(context)
            .run(Box::from(EmptyState));
    }
}
