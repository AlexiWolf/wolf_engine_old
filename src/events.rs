use std::sync::mpsc::{channel, Receiver, Sender};

pub struct EventQueue<E> {
    sender: Sender<E>,
    receiver: Receiver<E>,
}

impl<E> EventQueue<E> {
    pub fn new() -> Self {
        let (sender, receiver) = channel();
        Self { sender, receiver }
    }

    pub fn send(&self, event: E) {
        self.sender.send(event).unwrap();
    }

    pub fn sender(&self) -> Sender<E> {
        self.sender.clone()
    }

    pub fn flush(&self) -> Vec<E> {
        self.receiver.try_iter().collect()
    }
}

#[cfg(test)]
mod event_queue_tests {
    use std::thread;

    pub use super::*;

    #[test]
    pub fn should_send_and_receive_events() {
        let event_queue = EventQueue::new();

        event_queue.send(0);
        let events = event_queue.flush();

        assert_eq!(events.get(0).expect("No event in the queue"), &0);
    }
    
    #[test]
    pub fn should_send_events_through_a_sender() {
        let event_queue = EventQueue::new();
        let sender = event_queue.sender();
    
        sender.send(0).unwrap();
        thread::spawn(move || {
            sender.send(1).unwrap();
        })
            .join()
            .unwrap();
        
        let events = event_queue.flush();
        assert_eq!(events.get(0).unwrap(), &0);
        assert_eq!(events.get(1).unwrap(), &1);
    }

    #[test]
    pub fn should_flush_empty_list_if_there_are_no_events() {
        let event_queue = EventQueue::<i32>::new();

        assert!(event_queue.flush().is_empty());
    }
}
