use winit::{event_loop::{EventLoop, ControlFlow}, platform::run_return::EventLoopExtRunReturn, event::*};

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
    state_machine: StateStack,
}

impl<Loop: GameLoop> WolfEngine<Loop> {
    pub fn run(mut self, initial_state: Box<dyn State>, event_loop: EventLoop<()>) {
        self.state_machine.push(initial_state);
        self.run_event_loop(event_loop);
    }

    fn run_event_loop(&mut self, mut event_loop: EventLoop<()>) {
        event_loop.run_return(|event, _window, control_flow| {
            match event {
                Event::MainEventsCleared => {
                    self.game_loop.update(&mut self.context, &mut self.state_machine);
                },
                Event::RedrawRequested(_) => {
                    self.game_loop.render(&mut self.context, &mut self.state_machine);
                },
                _ => (),
            }
            if self.state_machine.is_empty() {
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
            state_machine: StateStack::new(),
        }
    }
}
