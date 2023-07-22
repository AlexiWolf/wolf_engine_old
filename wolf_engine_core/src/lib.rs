//! Provides common tools, types, and functions for the engine.
//!
//! The Core API provides all the parts likely to be (re)used by other parts of the engine.  It is 
//! mostly intended for those building, or making extensions to Wolf Engine, but there are some 
//! tools for end-users as well.
//!
//! # Getting Started
//!
//! TODO: Flesh out the docs into a useful guide to the core API.  This will be done after
//! currently-planned overhauls are made.

mod context;
pub use context::*;
mod engine;
pub use engine::*;
mod engine_controls;
pub use engine_controls::*;

pub mod events;

#[cfg(feature = "logging")]
pub mod logging;

#[doc(hidden)]
pub mod prelude {
    pub use super::*;
    pub use events::*;
}
