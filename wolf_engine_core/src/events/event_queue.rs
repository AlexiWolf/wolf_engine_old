use std::sync::Arc;

/// A generic, FIFO event queue.
///
/// The `EventQueue` uses a split receiver / sender design, similar to [`std::sync::mpsc::channel`]
/// where events are sent in by an associated [`EventSender`].  The order of incoming events is
/// always preserved.
///
/// # Examples
///
/// Iterating over all events using a `while let` loop.
///
/// ```
/// # use wolf_engine_core::events::*;
/// #
/// # let mut event_queue = MpscEventQueue::<Event>::new();
/// while let Some(event) = event_queue.next_event() {
///     // Do something cool.
/// }
/// ```
///
/// Sending an event through an [`EventSender`].
///
/// ```
/// # use wolf_engine_core::events::*;
/// #
/// # enum EventType { Event }
/// #
/// # let event_queue = MpscEventQueue::<EventType>::new();
/// let event_sender = event_queue.event_sender();
/// event_sender.send_event(EventType::Event);
/// ```
///
/// Sending an event across threads.
///
/// ```
/// # use wolf_engine_core::events::*;
/// #
/// # enum EventType { Event };
/// #
/// # let event_queue = MpscEventQueue::new();
/// let event_sender = event_queue.event_sender();
///
/// std::thread::spawn(move || {
///     event_sender.send_event(EventType::Event).unwrap();
/// })
/// # .join()
/// # .unwrap();
/// ```
pub trait EventQueue<E>: HasEventSender<E> {
    /// Returns the next event in the queue.
    fn next_event(&mut self) -> Option<E>;
}

/// A type which has an [`EventSender`].
pub trait HasEventSender<E> {
    /// Creates a new [`EventSender`].
    fn event_sender(&self) -> Arc<dyn EventSender<E>>;
}

/// A thread-safe link to an `EventQueue` for sending events.
pub trait EventSender<E>: Send + Sync {
    fn send_event(&self, event: E) -> Result<(), String>;
}
