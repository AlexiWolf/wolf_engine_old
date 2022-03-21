use std::{pin::Pin, sync::Arc};

use crate::event::{EventQueue, EventReader};

pub struct EventContext<E> {
    event_queue: Pin<Arc<EventQueue<E>>>,
}

impl<E> EventContext<E> {
    pub fn new() -> Self {
        let event_context = Self::default();
        event_context
    }

    pub fn push(&self, event: E) {
        self.truncate_queue_if_over_max_capacity(); 
        self.event_queue.push(event);
    }

    fn truncate_queue_if_over_max_capacity(&self) {
        if self.event_queue.total_capacity() < 100_000 {
            self.event_queue.truncate_front(1000);
        }
    }

    pub fn reader(&self) -> EventReader<E> {
        EventReader::new(&self.event_queue)
    }
}

impl<E> Default for EventContext<E> {
    fn default() -> Self {
        Self { 
            event_queue: EventQueue::new(), 
        }
    }
}

#[cfg(test)]
mod event_context_tests {
    use std::fmt::Debug;

    use crate::event::LendingIterator;

    pub use super::*;

    #[test]
    fn should_push_events_to_queue() {
        let events = EventContext::<u32>::default();
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
        let events = EventContext::<u32>::new(20_000);

        // Unused readers normally result in a memory leak.
        #[allow(unused)]
        let _unused_reader = events.reader();

        for i in 0..=50_000 {
            // The push method is expected to prevent memory leaks by limiting the event 
            // queue size.
            events.push(i);
        }

        assert!(events.event_queue.total_capacity() < 100_000, "The event queue exceeded 100_000 events");
        
    }
}
