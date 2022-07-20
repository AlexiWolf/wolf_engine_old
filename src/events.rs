use std::sync::mpsc::{Sender, Receiver, channel};

pub struct EventQueue<E> {
    sender: Sender<E>,
    receiver: Receiver<E>,
}

impl<E> EventQueue<E> {
    pub fn new() -> Self {
        let (sender, receiver) = channel();
        Self {
            sender,
            receiver,
        }
    }

    pub fn send(&self, event: E) {}
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
