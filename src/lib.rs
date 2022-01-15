//! Wolf Engine is a game framework for Rust with a focus on flexibility and ease of
//! use.   It aims to provide sensible default workflows to those who just want to 
//! build a game while allowing custom options for those who don't want to be forced
//! to do things *The Wolf Engine Way (TM)*.  
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
//! [FixedUpdateGameLoop](crate::game_loop::FixedUpdateGameLoop).  The default settings 
//! should be okay for most games.
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
//! If you want to customize the [FixedUpdateGameLoop](crate::game_loop::FixedUpdateGameLoop), 
//! you can build an instance yourself using the
//! [FixedUpdateGameLoopBuilder](crate::game_loop::FixedUpdateGameLoopBuilder), then pass 
//! it to `WolfEngineBuilder::with_fixed_game_loop()`.
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
//! To run the engine, you provide your game state to the engine.  The engine will use
//! the [GameLoop](crate::game_loop::GameLoop) to manage how the functions are called.  
//! The engine will take ownership over itself and run until the game quits.  You will
//! need to [Box] the state before passing it to the engine.
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
//! # Examples
//!
//! Refer to the [examples folder](https://github.com/AlexiWolf/wolf_engine/tree/main/examples)
//! for more complete examples of how to use Wolf Engine.

mod engine;
mod logging;
mod state;

pub mod context;
pub mod game_loop;

pub use context::{Context, ContextBuilder};
pub use engine::*;
pub use logging::*;
pub use state::*;
