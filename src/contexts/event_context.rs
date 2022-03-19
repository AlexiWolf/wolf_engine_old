use std::{pin::Pin, sync::Arc};

use rc_event_queue::mpmc::{EventQueue, DefaultSettings, EventReader};

pub struct EventContext<E> {
    event_queue: Pin<Arc<EventQueue<E>>> 
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

    pub fn reader(&self) -> EventReader<E, DefaultSettings> {
        EventReader::new(&*self.event_queue) 
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
        
        events.push(1);
        
        assert_next_event_equals(&events, &1);
    }

    fn assert_next_event_equals<E: Eq + PartialEq + Debug>(events: &EventContext<E>, expected: &E) {
        let mut reader = events.reader(); 
        while let Some(event) = reader.iter().next() {
            assert_eq!(event, expected, "the events do not match");
            return
        }
        panic!("No events received")
    }
}
