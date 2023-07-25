//! Provides common tools, types, and functions for the engine.
//!
//! The Core API provides all the parts likely to be (re)used by other parts of the event_loop.  It is
//! mostly intended for those building, or making extensions to Wolf Engine, but there are some
//! tools for end-users as well.
//!
//! # Getting Started
//!
//! TODO: Flesh out the docs into a useful guide to the core API.  This will be done after
//! currently-planned overhauls are made.

mod context;
pub use context::*;
mod event_loop;
pub use event_loop::*;

pub mod events;

#[cfg(feature = "logging")]
pub mod logging;

#[doc(hidden)]
pub mod prelude {
    pub use super::*;
    pub use events::*;
}

/// Initializes a new instance of the [`EventLoop`], and its associated [`Context`], with the
/// provided data.
///
/// #  Examples
///
/// ```
/// # use wolf_engine_core as wolf_engine;
/// # pub struct SomeData {};
/// # let some_data = SomeData {};
/// #
/// // The prelude brings in commonly needed types, and traits.
/// use wolf_engine::prelude::*;
///
/// // Start by initializing the EventLoop, and Context.
/// let (mut event_loop, mut context) = wolf_engine::init(some_data);
/// 
/// // Then, you can use the EventLoop to run your game's main-loop.
/// while let Some(event) = event_loop.next_event() {
///     // Do something cool!
/// #   break;
/// }
/// ```
pub fn init<D>(data: D) -> (EventLoop, Context<D>) {
    let event_loop = EventLoop::new();
    let context = Context::new(&event_loop, data);
    (event_loop, context)
}
