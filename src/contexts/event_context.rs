use std::{pin::Pin, sync::Arc};

use rc_event_queue::mpmc::EventQueue;

pub struct EventContext<E> {
    event_queue: Pin<Arc<EventQueue<E>>> 
}

impl<E> EventContext<E> {
    pub fn new() -> Self {
        Self {
            event_queue: EventQueue::new(),
        }
    }
}

#[cfg(test)]
mod event_context_tests {
    use crate::Context;

    pub use super::*;
    
    #[test]
    fn should_push_events_to_queue() {
        let events = EventContext::<u32>::new();
    }
}
