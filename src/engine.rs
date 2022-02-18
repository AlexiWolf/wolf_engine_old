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
///
/// For the default instance, just use:
///
/// ```
/// # use wolf_engine::{Engine, EmptyState};
/// # let my_game_state = EmptyState;
///
/// Engine::new()
///     .run(Box::from(my_game_state));
/// ```
pub struct Engine<Schedule: Scheduler> {
    context: Context,
    scheduler: Schedule,
    state_stack: StateStack,
}

impl<Loop: Scheduler> Engine<Loop> {
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
impl<Loop: Scheduler> Engine<Loop> {
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

/// Build and customize an instance of the [Engine].
pub struct EngineBuilder<Loop: Scheduler> {
    scheduler: Loop,
}

impl EngineBuilder<FixedUpdateScheduler> {
    pub fn with_default_scheduler() -> Self {
        Self {
            scheduler: Default::default(),
        }
    }

    pub fn with_fixed_scheduler(scheduler: FixedUpdateScheduler) -> Self {
        Self { scheduler }
    }
}

impl<Loop: Scheduler> EngineBuilder<Loop> {
    pub fn with_custom_scheduler(scheduler: Loop) -> Self {
        Self { scheduler }
    }

    pub fn build(self, context: Context) -> Engine<Loop> {
        Engine {
            context,
            scheduler: self.scheduler,
            state_stack: StateStack::new(),
        }
    }
}

#[cfg(test)]
mod wolf_engine_tests {
    use crate::{ContextBuilder, MockState, Transition};

    use super::*;

    #[test]
    fn should_run_the_state() {
        let context = ContextBuilder::new().build();
        let wolf_engine = EngineBuilder::with_default_scheduler().build(context);
        let mut state = MockState::new();
        state
            .expect_update()
            .times(1..)
            .returning(|_| Some(Transition::Quit));
        state.expect_render().times(1..).returning(|_| ());

        wolf_engine.run(Box::from(state));
    }
}
