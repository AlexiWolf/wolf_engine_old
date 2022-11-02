/// Provides the main event-loop functions used by the [`Engine`](crate::Engine).
pub trait EventLoop<E> {
    /// Returns the next event in the loop.
    fn next_event(&mut self) -> Option<E>;

    /// Send an event into the loop.
    fn send_event(&self, event: E);
}
