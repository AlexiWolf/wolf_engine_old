/// A type which can send events.  Most commonly used to send events back to an 
/// [`EventLoop`](super::EventLoop).
///
/// In cases where you need to send events across threads, or you need to skirt around borrowing
/// rules, using an [`EventSenderProxy`] may be needed.
pub trait EventSender<E> {
    fn send_event(&self, event: E) -> Result<(), String>;
}

/// A thread-safe link to an [`EventSender`].
pub trait EventSenderProxy<E>: EventSender<E> + Send + Sync {}
