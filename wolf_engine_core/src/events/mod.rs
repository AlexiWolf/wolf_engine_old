//! Provides an event system for the engine.
//!
//! Wolf Engine re-exports [Generic Event Queue](generic_event_queue), see the original crate for
//! details.

pub use generic_event_queue::*;

mod engine_events;
pub use engine_events::*;
