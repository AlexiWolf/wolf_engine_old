use std::sync::Arc;

/// A generic, FIFO event queue.
///
/// The `EventQueue` uses a split receiver / sender design, similar to [`std::sync::mpsc::channel`] 
/// where events are sent in by an associated [`EventSender`](super::EventSender) or 
/// [`EventSenderProxy`].  The order of incoming events is always preserved.
pub trait EventQueue<E>: HasEventSenderProxy<E> {
    /// Returns the next event in the queue.
    fn next_event(&mut self) -> Option<E>;
}

/// A type from which an [`EventSenderProxy`](super::EventSenderProxy) can be created.
pub trait HasEventSenderProxy<E> {
    /// Create a new [`EventSenderProxy`](super::EventSenderProxy).
    fn event_sender(&self) -> Arc<dyn EventSender<E>>;
}

/// A type which can send events.  Most commonly used to send events back to an 
/// [`EventQueue`].
///
/// In cases where you need to send events across threads, or you need to skirt around borrowing
/// rules, using an [`EventSenderProxy`] may be needed.
pub trait EventSender<E>: Send + Sync {
    fn send_event(&self, event: E) -> Result<(), String>;
}
