use std::sync::Arc;

/// Provides the main event-loop functions used by the [`Engine`](crate::Engine).
pub trait EventLoop<E>: HasEventSender<E> {
    /// Returns the next event in the loop.
    fn next_event(&mut self) -> Option<E>;

    /// Send an event into the loop.
    fn send_event(&self, event: E);
}

/// A type from which an [EventSender] can be created.
pub trait HasEventSender<E> {
    /// Create a new [`EventSender`] from this `EventLoop`.
    fn sender(&self) -> Arc<dyn EventSender<E>>;
}

/// A thread-safe proxy for sending events to the associated [`EventLoop`].
///
/// There may be many copies of an `EventSender`, and the [`EventLoop`] may have any number of
/// event senders associated with it.  An `EventSender` is valid as long as the connected
/// [`EventLoop`] hasn't been dropped.  The `EventSender` can be cloned, and passed around freely,
/// even across thread boundaries.
pub trait EventSender<E>: Send + Sync {
    /// Send an event to the associated [`EventLoop`].
    ///
    /// This method will return [`Ok`] if the event is sent successfully.  An [`Err`] normally
    /// means the [`EventLoop`] has been dropped, or is no longer reachable for any reason.  The
    /// error message should provide more details.
    fn send(&self, event: E) -> Result<(), String>;
}
