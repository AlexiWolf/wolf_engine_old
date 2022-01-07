use crate::{
    game_loop::{FixedUpdateGameLoop, GameLoop},
    Context,
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
}

impl<Loop: GameLoop> WolfEngine<Loop> {
    pub fn run(self) {}
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
        }
    }
}

