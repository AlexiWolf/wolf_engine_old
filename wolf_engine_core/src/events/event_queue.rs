use std::sync::Arc;

/// A generic, FIFO event queue.
///
/// The `EventQueue` uses a split receiver / sender design, similar to [`std::sync::mpsc::channel`] 
/// where events are sent in by an associated [`EventSender`](super::EventSender) or 
/// [`EventSenderProxy`].  The order of incoming events is always preserved.
pub trait EventQueue<E>: HasEventSender<E> {
    /// Returns the next event in the queue.
    fn next_event(&mut self) -> Option<E>;
}

/// A type from which an [`EventSenderProxy`](super::EventSenderProxy) can be created.
pub trait HasEventSender<E> {
    /// Create a new [`EventSenderProxy`](super::EventSenderProxy).
    fn event_sender(&self) -> Arc<dyn EventSender<E>>;
}

/// A thread-safe link to an `EventQueue` for sending events.
pub trait EventSender<E>: Send + Sync {
    fn send_event(&self, event: E) -> Result<(), String>;
}
