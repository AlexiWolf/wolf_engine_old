use std::sync::mpsc::*;
use std::sync::Arc;

use crate::events::*;

/// Provides a MPSC [`EventQueue`] implementation based on [`std::sync::mpsc`].
pub struct MpscEventQueue<E> {
    sender: Sender<E>,
    receiver: Receiver<E>,
}

impl<E> MpscEventQueue<E> {
    /// Creates a new event queue.
    pub fn new() -> Self {
        let (sender, receiver) = channel();
        Self { sender, receiver }
    }
}

impl<E: 'static> EventQueue<E> for MpscEventQueue<E> {
    fn next_event(&mut self) -> Option<E> {
        self.receiver.try_recv().ok()
    }
}

impl<E: 'static> HasEventSender<E> for MpscEventQueue<E> {
    fn event_sender(&self) -> Arc<dyn EventSender<E>> {
        Arc::from(MpscEventQueueSender::from(self.sender.clone()))
    }
}

impl<E> Default for MpscEventQueue<E> {
    fn default() -> Self {
        Self::new()
    }
}

struct MpscEventQueueSender<E> {
    inner: Sender<E>,
}

unsafe impl<E> Send for MpscEventQueueSender<E> {}
unsafe impl<E> Sync for MpscEventQueueSender<E> {}

impl<E> From<Sender<E>> for MpscEventQueueSender<E> {
    fn from(sender: Sender<E>) -> Self {
        Self { inner: sender }
    }
}

impl<E> EventSender<E> for MpscEventQueueSender<E> {
    fn send_event(&self, event: E) -> Result<(), String> {
        match self.inner.send(event) {
            Ok(_) => Ok(()),
            Err(error) => Err(error.to_string()),
        }
    }
}

#[cfg(test)]
mod event_queue_tests {
    use std::thread;

    pub use super::*;

    #[test]
    pub fn should_send_and_receive_events() {
        let mut event_queue = MpscEventQueue::new();
        let event_sender = event_queue.event_sender();

        event_sender.send_event(0).unwrap();

        assert_eq!(event_queue.next_event().expect("No event in the queue"), 0);
    }

    #[test]
    pub fn should_send_events_and_receive_events_across_threads() {
        let mut event_queue = MpscEventQueue::new();
        let sender = event_queue.event_sender();

        sender.send_event(0).unwrap();
        let thread_sender = sender.clone();
        thread::spawn(move || {
            thread_sender.send_event(1).unwrap();
        })
        .join()
        .unwrap();
        sender.send_event(2).unwrap();

        assert_eq!(event_queue.next_event().expect("No event in the queue."), 0);
        assert_eq!(event_queue.next_event().expect("No event in the queue."), 1);
        assert_eq!(event_queue.next_event().expect("No event in the queue."), 2);
    }

    #[test]
    pub fn should_flush_empty_list_if_there_are_no_events() {
        let mut event_queue = MpscEventQueue::<i32>::new();

        assert!(event_queue.next_event().is_none());
    }

    #[test]
    pub fn should_implement_default_trait() {
        let _event_queue = MpscEventQueue::<i32>::default();
    }
}
