use crate::{Context, FixedUpdateGameLoop, FixedUpdateGameLoopBuilder, GameLoop};

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
/// To initialize the engine, start by initializing a [Context] using the 
/// [ContextBuilder](crate::ContextBuilder).
///
/// ```
/// # use wolf_engine::ContextBuilder;
/// #
/// let context = ContextBuilder::new()
///     // Custom settings.
///     .build();
///```
/// 
/// Then you can build and instance of the engine using the [WolfEngineBuilder]. 
/// The `WolfEngineBuilder::with_default_game_loop()` method will give you the default 
/// [FixedUpdateGameLoop]. The default settings should be okay for most games.
/// 
/// ```
/// # use wolf_engine::{ContextBuilder, WolfEngineBuilder};
/// #
/// # let context = ContextBuilder::new()
/// #    .build(); 
/// #
/// let engine = WolfEngineBuilder::with_default_game_loop()
///     .build(context);
/// ```
/// 
/// If you want to customize the [FixedUpdateGameLoop], you can build an instance yourself using the
/// [FixedUpdateGameLoopBuilder](crate::FixedUpdateGameLoopBuilder), then pass it to 
/// `WolfEngineBuilder::with_fixed_game_loop()`.
/// 
/// ```
/// # use wolf_engine::{ContextBuilder, WolfEngineBuilder, FixedUpdateGameLoopBuilder};
/// #
/// # let context = ContextBuilder::new()
/// #    .build(); 
/// #
/// let game_loop = FixedUpdateGameLoopBuilder::new()
///     // Custom settings.
///     .build();
/// 
/// let engine = WolfEngineBuilder::with_fixed_game_loop(game_loop)
///     .build(context);
/// ```
/// 
/// Alternatively, you can use a custom [GameLoop] implementation by using the 
/// `WolfEngineBuilder::with_custom_game_loop()` method.
/// 
/// Refer to the [GameLoop] documentation for more information on implementing a custom game loop.
/// 
/// ```
/// # use wolf_engine::{ContextBuilder, WolfEngineBuilder, FixedUpdateGameLoopBuilder};
/// #
/// # let context = ContextBuilder::new()
/// #    .build(); 
/// #
/// # // For demonstrational purposes, this game loop will work, but it can be any GameLoop.
/// # let custom_game_loop = FixedUpdateGameLoopBuilder::new()
/// #   .build();
/// #
/// let engine = WolfEngineBuilder::with_custom_game_loop(custom_game_loop)
///     .build(context);
/// ```
/// 
/// To run the engine, you provide the `update` and `render` functions for your game.  The engine
/// will use the [GameLoop] to manage how the functions are called.  The engine will take ownership
/// over itself and run until the game quits.
///
/// ```
/// # use wolf_engine::{WolfEngine, WolfEngineBuilder, FixedUpdateGameLoop, ContextBuilder};
/// #
/// # let context = ContextBuilder::new()
/// #    .build(); 
/// #
/// # let engine = WolfEngineBuilder::with_default_game_loop()
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
    pub fn with_default_game_loop() -> Self {
        Self {
            game_loop: Default::default()
        }
    }

    pub fn with_fixed_game_loop(game_loop: FixedUpdateGameLoop) -> Self{
        Self {
            game_loop
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
