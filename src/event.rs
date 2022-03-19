use rc_event_queue::mpmc::DefaultSettings;

pub use rc_event_queue::LendingIterator;

pub type EventReaderSettings = DefaultSettings;
pub type EventQueue<E> = rc_event_queue::mpmc::EventQueue<E>;
pub type EventReader<E> = rc_event_queue::mpmc::EventReader<E, EventReaderSettings>;

