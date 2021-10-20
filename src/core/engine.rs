use crate::{Context, FixedUpdateGameLoop, GameLoop};

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
///     |_context| {
///         // Update Function
///         std::process::exit(0)
///     },
///     |_context| {
///         // Render Function
///     },
/// );
/// ```
pub struct WolfEngine<Loop: GameLoop> {
    game_loop: Loop,
}

impl<Loop: GameLoop> WolfEngine<Loop> {
    pub fn run<Update, Render>(self, update_function: Update, render_function: Render)
    where
        Update: FnMut(&mut Context),
        Render: FnMut(&mut Context),
    {
        loop {
            self.game_loop.update(&mut self.context, update_function);
            self.game_loop.render(&mut self.context, render_function);
        }
    }
}

pub struct WolfEngineBuilder<Loop: GameLoop> {
    engine: WolfEngine<Loop>,
}

impl WolfEngineBuilder<FixedUpdateGameLoop> {
    pub fn with_default_game_loop() -> Self {
        let game_loop = FixedUpdateGameLoop::default();
        let engine = WolfEngine { game_loop };
        Self { engine }
    }
}

impl<Loop: GameLoop> WolfEngineBuilder<Loop> {
    pub fn build(self) -> WolfEngine<Loop> {
        self.engine
    }
}
