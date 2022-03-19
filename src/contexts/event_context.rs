use std::{pin::Pin, sync::Arc};

use rc_event_queue::mpmc::DefaultSettings;

pub type EventQueue<E> = rc_event_queue::mpmc::EventQueue<E>;
pub type EventReader<E> = rc_event_queue::mpmc::EventReader<E, DefaultSettings>;

pub struct EventContext<E> {
    event_queue: Pin<Arc<EventQueue<E>>>,
}

impl<E> EventContext<E> {
    pub fn new() -> Self {
        Self {
            event_queue: EventQueue::new(),
        }
    }

    pub fn push(&self, event: E) {
        self.event_queue.push(event);
    }

    pub fn reader(&self) -> EventReader<E> {
        EventReader::new(&self.event_queue)
    }
}

#[cfg(test)]
mod event_context_tests {
    use std::fmt::Debug;

    use rc_event_queue::LendingIterator;

    pub use super::*;

    #[test]
    fn should_push_events_to_queue() {
        let events = EventContext::<u32>::new();
        let mut reader = events.reader();

        events.push(1);

        assert_next_event_equals(&mut reader, &1);
    }

    fn assert_next_event_equals<E: Eq + PartialEq + Debug>(
        reader: &mut EventReader<E>,
        expected: &E,
    ) {
        while let Some(event) = reader.iter().next() {
            assert_eq!(event, expected, "the events do not match");
            return;
        }
        panic!("No events received")
    }
}
