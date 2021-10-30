use crate::{Context, ContextBuilder, FixedUpdateGameLoop, GameLoop};

/// The main entry-point for the engine.  Start here!
///
/// This struct provides the core of the engine, it has a [GameLoop], and a [Context].  The engine
/// runs the main loop and uses the methods provided by the [GameLoop] to control how the update
/// and render functions are called.  
///
/// # Examples
///
/// Using the default game loop.
///
/// ```
/// # use wolf_engine::{WolfEngine, WolfEngineBuilder, FixedUpdateGameLoop};
/// #
/// let engine: WolfEngine<FixedUpdateGameLoop> = WolfEngineBuilder::with_default_game_loop()
///     .build();
/// ```
///
/// To run the engine, you provide the `update` and `render` functions for your game.  The engine
/// will use the [GameLoop] to manage how the functions are called.  The engine will take ownership
/// over itself and run until the game quits.
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
///         # std::process::exit(0);
///     },
///     |_context| {
///         // Render Function
///     },
/// );
/// ```
pub struct WolfEngine<Loop: GameLoop> {
    context: Context,
    game_loop: Loop,
}

impl<Loop: GameLoop> WolfEngine<Loop> {
    pub fn run<Update, Render>(mut self, mut update_function: Update, mut render_function: Render)
    where
        Update: FnMut(&mut Context),
        Render: FnMut(&mut Context),
    {
        loop {
            self.game_loop
                .update(&mut self.context, |context| update_function(context));
            self.game_loop
                .render(&mut self.context, |context| render_function(context));
        }
    }
}

/// Builds an instance of [WolfEngine].
pub struct WolfEngineBuilder<Loop: GameLoop> {
    engine: WolfEngine<Loop>,
}

impl WolfEngineBuilder<FixedUpdateGameLoop> {
    pub fn with_default_game_loop() -> Self {
        let game_loop = FixedUpdateGameLoop::default();
        let context = ContextBuilder::new().build();
        let engine = WolfEngine { game_loop, context };
        Self { engine }
    }
}

impl<Loop: GameLoop> WolfEngineBuilder<Loop> {
    pub fn build(self) -> WolfEngine<Loop> {
        self.engine
    }
}
