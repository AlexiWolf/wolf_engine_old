use std::sync::Arc;

/// Provides the main event-loop functions used by the [`Engine`](crate::Engine).
pub trait EventLoop<E> {
    /// Returns the next event in the loop.
    fn next_event(&mut self) -> Option<E>;

    /// Send an event into the loop.
    fn send_event(&self, event: E);

    fn sender(&self) -> Arc<dyn EventSender<E>>;
}

pub trait EventSender<E>: Send + Sync {
    fn send(&self, event: E) -> Result<(), String>;
}
