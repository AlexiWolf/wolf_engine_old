//! Provides default engine events and an event system backed by [rc_event_queue].

pub mod rc_event_queue {
    //! Reexports [rc_event_queue]. 
    pub use rc_event_queue::*;
}

use ::rc_event_queue::mpmc::DefaultSettings;

pub use ::rc_event_queue::LendingIterator;

pub type EventReaderSettings = DefaultSettings;
pub type EventQueue<E> = ::rc_event_queue::mpmc::EventQueue<E>;
pub type EventReader<E> = ::rc_event_queue::mpmc::EventReader<E, EventReaderSettings>;

