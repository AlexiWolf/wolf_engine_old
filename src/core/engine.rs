use crate::{Context, ContextBuilder, FixedUpdateGameLoop, FixedUpdateGameLoopBuilder, GameLoop};

/// Provides the core functionality of the engine.
/// 
/// `WolfEngine` is, as the name suggests, the core of the game engine.  It provides some common
/// behavior such as: Running the main loop (utilizing a [GameLoop] for timing control), cleanly
/// shutting down, and holding ownership over the [Context] object.
/// 
/// The tries to only include functionality that is common to all `WolfEngine` projects.  Anything
/// else should live on the [Context] object instead.
/// 
/// # Examples
/// 
/// Initializing the Engine.
/// 
/// First, start by initializing a [Context] using the [ContextBuilder].
///
/// ```
/// # use wolf_engine::ContextBuilder;
/// #
/// let context = ContextBuilder::new()
///     .build();
///```
/// 
/// Then you can build and instance of the engine using the [WolfEngineBuilder].  The `build()` 
/// method will take ownership over the [Context].
/// 
/// ```
/// # use wolf_engine::{ContextBuilder, WolfEngineBuilder};
/// #
/// # let context = ContextBuilder::new()
/// #    .build(); 
/// #
/// let engine = WolfEngineBuilder::with_fixed_game_loop()
///     .build(context);
/// ```
///
/// To run the engine, you provide the `update` and `render` functions for your game.  The engine
/// will use the [GameLoop] to manage how the functions are called.  The engine will take ownership
/// over itself and run until the game quits.
///
/// ```
/// # use wolf_engine::{WolfEngine, WolfEngineBuilder, FixedUpdateGameLoop};
/// #
/// # let context = ContextBuilder::new()
/// #    .build(); 
/// #
/// # let engine: WolfEngine<FixedUpdateGameLoop> = WolfEngineBuilder::with_fixed_game_loop()
/// #    .build(context);
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

/// Build an instance of [WolfEngine].
pub struct WolfEngineBuilder<Loop: GameLoop> {
    game_loop: Loop
}

impl WolfEngineBuilder<FixedUpdateGameLoop> {
    pub fn with_fixed_game_loop() -> Self {
        Self {
            game_loop: Default::default()
        }
    }    
}

impl<Loop: GameLoop> WolfEngineBuilder<Loop> {
    pub fn build(self, context: Context) -> WolfEngine<Loop> {
        WolfEngine {
            context, 
            game_loop: self.game_loop 
        }
    }
}
