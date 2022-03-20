use std::{pin::Pin, sync::Arc};

use crate::event::{EventQueue, EventReader};

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

    use crate::event::LendingIterator;

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

    #[test]
    fn should_prevent_memory_leaks_by_dropping_events_from_the_queue() {
        let events = EventContext::<u32>::new();

        // Unused readers normally result in a memory leak.
        #[allow(unused)]
        let _unused_reader = events.reader();

        for i in 0..=200_000 {
            // The push method is expected to prevent memory leaks by limiting the event 
            // queue size.
            events.push(i);
        }

        assert!(events.event_queue.total_capacity() < 100_000, "The event queue exceeded 100_000 events");
        
    }
}
