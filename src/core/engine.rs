use crate::{FixedUpdateGameLoop, GameLoop};


/// Provides access to all engine state.
/// 
/// # Examples
/// 
/// Using the default game loop.
/// 
/// ```
/// # use wolf_engine::{WolfEngine, WolfEngineBuilder, FixedUpdateGameLoop};
/// 
/// let engine: WolfEngine<FixedUpdateGameLoop> = WolfEngineBuilder::with_default_game_loop()
///     .build();
/// ```
/// 
/// Running the engine.
/// 
/// ```
/// # use wolf_engine::{WolfEngine, WolfEngineBuilder, FixedUpdateGameLoop};
/// #
/// # let engine: WolfEngine<FixedUpdateGameLoop> = WolfEngineBuilder::with_default_game_loop()
/// #    .build();
/// #
/// engine.run(
///     |&mut context| {
///         // Update Function
///         std::process::exit(0)
///     },
///     |&mut context| {
///         // Render Function
///     },
/// );
/// ```
pub struct WolfEngine<Loop: GameLoop> {
    game_loop: Loop
}

pub struct WolfEngineBuilder<Loop: GameLoop> {
    engine: WolfEngine<Loop>
}

impl WolfEngineBuilder<FixedUpdateGameLoop> {
    pub fn with_default_game_loop() -> Self {
        
        let game_loop = FixedUpdateGameLoop::default();
        let engine = WolfEngine {
            game_loop,
        };
        Self {
            engine
        }
    }
}

impl<Loop: GameLoop> WolfEngineBuilder<Loop> {
    pub fn build(self) -> WolfEngine<Loop> {
        self.engine
    }
}