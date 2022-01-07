//! A lightweight game framework.
//!
//! # Getting Started
//!
//! To initialize the engine, start by initializing a [Context] using the
//! [ContextBuilder](crate::ContextBuilder).
//!
//! ```
//! # use wolf_engine::ContextBuilder;
//! #
//! let context = ContextBuilder::new()
//!     // Custom settings.
//!     .build();
//!```
//!
//! Then you can build and instance of the engine using the [WolfEngineBuilder].
//! The `WolfEngineBuilder::with_default_game_loop()` method will give you the default
//! [FixedUpdateGameLoop](crate::game_loop::FixedUpdateGameLoop).  The default settings should be
//! okay for most games.
//!
//! ```
//! # use wolf_engine::{ContextBuilder, WolfEngineBuilder};
//! #
//! # let context = ContextBuilder::new()
//! #    .build();
//! #
//! let engine = WolfEngineBuilder::with_default_game_loop()
//!     .build(context);
//! ```
//!
//! If you want to customize the [FixedUpdateGameLoop](crate::game_loop::FixedUpdateGameLoop), you
//! can build an instance yourself using the
//! [FixedUpdateGameLoopBuilder](crate::game_loop::FixedUpdateGameLoopBuilder), then pass it to
//! `WolfEngineBuilder::with_fixed_game_loop()`.
//!
//! ```
//! # use wolf_engine::{ContextBuilder, WolfEngineBuilder, game_loop::FixedUpdateGameLoopBuilder};
//! #
//! # let context = ContextBuilder::new()
//! #    .build();
//! #
//! let game_loop = FixedUpdateGameLoopBuilder::new()
//!     // Custom settings.
//!     .build();
//!
//! let engine = WolfEngineBuilder::with_fixed_game_loop(game_loop)
//!     .build(context);
//! ```
//!
//! # Starting the Engine
//!  
//! To run the engine, you provide your game state to the engine.  The engine will use 
//! the [GameLoop](crate::game_loop::GameLoop) to manage how the functions are called.  
//! The engine will take ownership over itself and run until the game quits.
//!
//! ```
//! # use wolf_engine::{EmptyState, WolfEngine, WolfEngineBuilder, ContextBuilder, game_loop::FixedUpdateGameLoop };
//! #
//! # let context = ContextBuilder::new()
//! #    .build();
//! #
//! # let engine = WolfEngineBuilder::with_default_game_loop()
//! #    .build(context);
//! #
//! # let state = EmptyState;
//! #
//! engine.run(Box::from(state));
//! ```
//!
//! ## Custom Game Loops
//!  
//! Alternatively, you can use a custom [GameLoop](crate::game_loop::GameLoop) implementation by
//! using the `WolfEngineBuilder::with_custom_game_loop()` method.
//!
//! Refer to the [GameLoop](crate::game_loop::GameLoop) documentation for more information on
//! implementing a custom game loop.
//!
//! ```
//! # use wolf_engine::{ContextBuilder, WolfEngineBuilder, game_loop::{FixedUpdateGameLoopBuilder}};
//! #
//! # let context = ContextBuilder::new()
//! #    .build();
//! #
//! # // For demonstrational purposes, this game loop will work, but it can be any GameLoop.
//! # let custom_game_loop = FixedUpdateGameLoopBuilder::new()
//! #   .build();
//! #
//! let engine = WolfEngineBuilder::with_custom_game_loop(custom_game_loop)
//!     .build(context);
//! ```

mod engine;
mod logging;
mod state;

pub mod context;
pub mod game_loop;

pub use context::{Context, ContextBuilder};
pub use engine::*;
pub use logging::*;
pub use state::*;
