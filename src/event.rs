use rc_event_queue::mpmc::DefaultSettings;

pub type EventQueue<E> = rc_event_queue::mpmc::EventQueue<E>;
pub type EventReader<E> = rc_event_queue::mpmc::EventReader<E, DefaultSettings>;

pub use rc_event_queue::prelude::*;

