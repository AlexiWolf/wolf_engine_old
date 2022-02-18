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
//! The `EngineBuilder::with_default_scheduler()` method will give you the default
//! [FixedUpdateScheduler](crate::scheduler::FixedUpdateScheduler).  The default settings
//! should be okay for most games.
//!
//! ```
//! # use wolf_engine::{ContextBuilder, EngineBuilder};
//! #
//! # let context = ContextBuilder::new()
//! #    .build();
//! #
//! let engine = EngineBuilder::with_default_scheduler()
//!     // Custom settings go here.
//!     .build(context);
//! ```
//!
//! If you want to customize the [FixedUpdateScheduler](crate::scheduler::FixedUpdateScheduler),
//! you can build an instance yourself using the
//! [FixedUpdateSchedulerBuilder](crate::scheduler::FixedUpdateSchedulerBuilder), then pass
//! it to `EngineBuilder::with_fixed_scheduler()`.
//!
//! ```
//! # use wolf_engine::{ContextBuilder, EngineBuilder, scheduler::FixedUpdateSchedulerBuilder};
//! #
//! # let context = ContextBuilder::new()
//! #    .build();
//! #
//! let scheduler = FixedUpdateSchedulerBuilder::new()
//!     // Custom settings go here.
//!     .build();
//!
//! let engine = EngineBuilder::with_fixed_scheduler(scheduler)
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
//! # let engine = EngineBuilder::with_default_scheduler()
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
pub mod scheduler;

pub use context::{Context, ContextBuilder};
pub use engine::*;
pub use logging::*;
pub use state::*;
