//! Provides common tools, types, and functions for the engine.
//!
//! The core module provides all the common components likely to be used by most projects.  It
//! provides some basic functionality, such as a main event-loop, but it is generally expected
//! users will implement their own architecture when using the core API.  The Core API is
//! re-exported by the main `wolf_engine` crate.

mod context;
pub use context::*;
mod engine;
pub use engine::*;

pub mod events;

#[cfg(feature = "logging")]
pub mod logging;

#[doc(hidden)]
pub mod prelude {
    pub use super::*;
    pub use events::EventLoop;
}
