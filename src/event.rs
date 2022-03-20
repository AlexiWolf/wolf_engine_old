//! Provides default engine events and an event system backed by [rc_event_queue].

pub mod rc_event_queue {
    //! Reexports [rc_event_queue]. 
    pub use rc_event_queue::*;
}

use ::rc_event_queue::mpmc::DefaultSettings;

pub use ::rc_event_queue::LendingIterator;

/// The [Settings](rc_event_queue::Settings) used by the [EventReader].
pub type EventReaderSettings = DefaultSettings;

/// Provides a FIFO event queue.
///
/// Events can be pushed into the event queue, then read back using an [EventReader].
///
/// The current event system uses [rc_event_queue] directly.  This typedef is provided to
/// help make the API more friendly to the engine.
pub type EventQueue<E> = ::rc_event_queue::mpmc::EventQueue<E>;

/// Provides a FIFO event reader.
///
/// The event reader is created from the [EventQueue] and can read events from it.
///
/// The current event system uses [rc_event_queue] directly.  This typedef is provided to
/// help make the API more friendly to the engine.
pub type EventReader<E> = ::rc_event_queue::mpmc::EventReader<E, EventReaderSettings>;

