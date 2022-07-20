pub struct EventQueue {}

impl EventQueue {
    pub fn new() -> Self {
        Self {}
    }
}

#[cfg(test)]
mod event_queue_tests {
   pub use super::*;

    #[test]
    pub fn should_send_and_receive_events() {
        let event_queue = EventQueue::new();

        event_queue.send(0);
    }
}
