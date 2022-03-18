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
        
    }

    pub fn reader(&self) -> EventReader<E, DefaultSettings> {
        EventReader::new(&*self.event_queue) 
    }
}

#[cfg(test)]
mod event_context_tests {
    use crate::Context;

    pub use super::*;
    
    #[test]
    fn should_push_events_to_queue() {
        let events = EventContext::<u32>::new();
        
        events.push(1);

        assert_eq!(get_next_event(&events), 1, "The event was not present");
    }

    fn get_next_event<E>(events: &EventContext<E>) -> E {
        events.reader().iter().next().expect("could not read an event")
    }
}
