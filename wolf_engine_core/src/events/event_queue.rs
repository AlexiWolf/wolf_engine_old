use std::sync::Arc;

/// A generic, FIFO, MPSC event queue.
///
/// A complete guide, with examples, on how use trait can be found in the main
/// [`events module`](crate::events) documentation.
pub trait EventQueue<E>: HasEventSender<E> {
    /// Returns the next event in the queue.
    fn next_event(&mut self) -> Option<E>;
}

/// A type which has an [`EventSender`].
///
/// A complete guide, with examples, on how use trait can be found in the main
/// [`events module`](crate::events) documentation.
pub trait HasEventSender<E> {
    /// Creates a new [`EventSender`].
    fn event_sender(&self) -> Arc<dyn EventSender<E>>;
}

/// The event-sending half of an [`EventQueue`].
///
/// Event Senders can be freely, and safely cloned, given away, and even sent across threads.
///
/// A complete guide, with examples, on how use trait can be found in the main
/// [`events module`](crate::events) documentation.
pub trait EventSender<E>: Send + Sync {
    fn send_event(&self, event: E) -> Result<(), String>;
}
