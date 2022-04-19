//! Wolf Engine is a game framework for Rust with a focus on flexibility and ease of
//! use.   It aims to provide sensible default workflows to those who just want to
//! build a game while allowing custom options for those who don't want to be forced
//! to do things *The Wolf Engine Way (TM)*.  
//!
//! # Getting Started
//!
//! Wolf Engine ships with sensible defaults to help jump-start projects as quickly as
//! possible.  You can get started with the default settings by calling [Engine::new()],
//! or alternatively [Engine::default()].
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
//! The defaults are probably fine for simple projects, or when you're just getting
//! started.  You can reference the [Engine]'s documentation if you want to customize the
//! engine.
//!
//! ## Game States
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

mod core;

pub mod contexts;
pub mod event;
pub mod plugins;
pub mod schedulers;
pub mod utils;

#[cfg(feature = "logging")]
pub mod logging;

pub use crate::core::*;

use log::info;

pub(crate) fn log_startup_information() {
    let name = env!("CARGO_PKG_NAME");
    let version = env!("CARGO_PKG_VERSION");
    let homepage = env!("CARGO_PKG_HOMEPAGE");
    info!("Hello from {} v{} - {}", name, version, homepage);
}

pub(crate) fn log_shutdown() {
    info!("Engine has stopped.  Goodbye.")
}
