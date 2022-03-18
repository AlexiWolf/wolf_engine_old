pub struct EventContext<E>;

impl<E> EventContext<E> {
    pub fn new() -> Self {
        Self
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
