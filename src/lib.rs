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
//! Then you can build and instance of the engine using the [EngineBuilder].
//! The [EngineBuilder::new()] method will give you the default
//! [FixedUpdateScheduler](crate::scheduler::FixedUpdateScheduler).  The default settings
//! should be okay for most games.
//!
//! ```
//! # use wolf_engine::{ContextBuilder, EngineBuilder};
//! #
//! # let context = ContextBuilder::new()
//! #    .build();
//! #
//! let engine = EngineBuilder::new()
//!     // Custom settings go here.
//!     .build(context);
//! ```
//!
//! To run the engine, you provide your game state to the engine.  The engine will use
//! the [Scheduler](crate::scheduler::Scheduler) to manage how the functions are called.  
//! The engine will take ownership over itself and run until the game quits.  You will
//! need to [Box] the state before passing it to the engine.
//!
//! ```
//! # use wolf_engine::{EmptyState, Engine, EngineBuilder, ContextBuilder, scheduler::FixedUpdateScheduler };
//! #
//! # let context = ContextBuilder::new()
//! #    .build();
//! #
//! # let engine = EngineBuilder::new()
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
mod state;

#[cfg(feature = "logging")]
mod logging;

pub mod core;
pub mod context;
pub mod scheduler;

pub use context::{Context, ContextBuilder};
pub use engine::*;
pub use state::*;

#[cfg(feature = "logging")]
pub use logging::*;
