//! Provides an event system for the engine.
//!
//! Wolf Engine uses a MPSC event system based on the sender / receiver model found in 
//! [std::sync::mpsc::channel]. 
//!

mod event_queue;
pub use event_queue::*;
mod mpsc_event_queue;
pub use mpsc_event_queue::*;
mod engine_events;
pub use engine_events::*;
