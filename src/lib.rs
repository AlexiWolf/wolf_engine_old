//! Wolf Engine is a game framework for Rust with a focus on flexibility and ease of
//! use.   It aims to provide sensible default workflows to those who just want to
//! build a game while allowing custom options for those who don't want to be forced
//! to do things *The Wolf Engine Way (TM)*.  
//!
//! # Getting Started
//!
//! If you don't care about customizing the engine's behavior, [Engine::new()] is the
//! fastest way to get up and running.
//!
//! ```
//! # use wolf_engine::{Engine, EmptyState};
//! #
//! # let my_game_state = EmptyState;
//! #
//! Engine::new()
//!     .run(Box::from(my_game_state));
//! ```
//!
//! Wolf Engine games are organized into one or more game [State]s.  These game [State]s
//! bundle your game's data and logic into a single package that's easy to pass to the
//! [Engine].  You will need to implement your game as a [State].
//!
//! ```
//! # use wolf_engine::*;
//! #
//! pub struct MyGame;
//!
//! impl State for MyGame {
//!     fn update(&mut self, _context: &mut Context) -> OptionalTransition {
//!         // Update your game here.
//!         None
//!     }
//!
//!     fn render(&mut self, _context: &mut Context) -> RenderResult {
//!         // Render your game here.
//!     }
//! }
//! ```
//!
//! Refer to the [examples folder](https://github.com/AlexiWolf/wolf_engine/tree/main/examples)
//! for more complete examples of how to use Wolf Engine.  The [Quick-start Example](https://github.com/AlexiWolf/wolf_engine/blob/main/examples/quickstart.rs)
//! is a good starting place.

mod engine;
mod state;

#[cfg(feature = "logging")]
mod logging;

pub mod context;
pub mod core;
pub mod scheduler;

pub use context::{Context, ContextBuilder};
pub use engine::*;
pub use state::*;

#[cfg(feature = "logging")]
pub use logging::*;
