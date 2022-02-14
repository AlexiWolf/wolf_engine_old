#[cfg(feature = "window")]
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    platform::run_return::EventLoopExtRunReturn,
};

use crate::{
    game_loop::{FixedUpdateGameLoop, GameLoop},
    Context, State, StateStack,
};

/// Provides the core functionality of the engine.
///
/// `WolfEngine` is, as the name suggests, the core of the game engine.  It provides some common
/// behavior such as: Running the main loop (utilizing a [GameLoop] for timing control), cleanly
/// shutting down, and holding ownership over the [Context] object.
///
/// The engine tries to only include functionality that is common to all `WolfEngine` projects.  
/// Anything else should live on the [Context] object instead.
pub struct WolfEngine<Loop: GameLoop> {
    context: Context,
    game_loop: Loop,
    state_stack: StateStack,
}

impl<Loop: GameLoop> WolfEngine<Loop> {
    pub fn run(mut self, initial_state: Box<dyn State>) {
        self.state_stack.push(initial_state);
        while !self.state_stack.is_empty() {
            self.game_loop
                .update(&mut self.context, &mut self.state_stack);
            self.game_loop
                .render(&mut self.context, &mut self.state_stack);
        }
    }
}

#[cfg(feature = "window")]
impl<Loop: GameLoop> WolfEngine<Loop> {
    pub fn run_with_event_loop(mut self, initial_state: Box<dyn State>, event_loop: EventLoop<()>) {
        self.state_stack.push(initial_state);
        self.run_event_loop(event_loop);
    }

    fn run_event_loop(&mut self, mut event_loop: EventLoop<()>) {
        event_loop.run_return(|event, _window, control_flow| {
            match event {
                Event::MainEventsCleared => {
                    self.game_loop
                        .update(&mut self.context, &mut self.state_stack);
                }
                Event::RedrawRequested(_) => {
                    self.game_loop
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

/// Build an instance of [WolfEngine].
pub struct WolfEngineBuilder<Loop: GameLoop> {
    game_loop: Loop,
}

impl WolfEngineBuilder<FixedUpdateGameLoop> {
    pub fn with_default_game_loop() -> Self {
        Self {
            game_loop: Default::default(),
        }
    }

    pub fn with_fixed_game_loop(game_loop: FixedUpdateGameLoop) -> Self {
        Self { game_loop }
    }
}

impl<Loop: GameLoop> WolfEngineBuilder<Loop> {
    pub fn with_custom_game_loop(game_loop: Loop) -> Self {
        Self { game_loop }
    }

    pub fn build(self, context: Context) -> WolfEngine<Loop> {
        WolfEngine {
            context,
            game_loop: self.game_loop,
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
        let wolf_engine = WolfEngineBuilder::with_default_game_loop().build(context);
        let mut state = MockState::new();
        state
            .expect_update()
            .times(1..)
            .returning(|_| Some(Transition::Quit));
        state.expect_render().times(1..).returning(|_| ());

        wolf_engine.run(Box::from(state));
    }
}
