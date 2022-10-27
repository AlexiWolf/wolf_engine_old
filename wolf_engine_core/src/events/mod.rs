//! Provides an event system for the engine.

mod event_queue;
pub use event_queue::*;

use crate::*;

/// Provides a set of convenience methods to aid in working with multiple [EventQueues](EventQueue).
///
/// The main intention for this trait is to provide additional methods to [Context], allowing users
/// to avoid having to manually borrow the [EventQueue] they want to work with.  For example,
/// instead of:
///
/// ```
/// # use wolf_engine_core::*;
/// # use wolf_engine_core::events::*;
/// #
/// # let mut context = Context::new();
/// # let event_queue = EventQueue::<i32>::new();
/// # context.add(event_queue);
/// #
/// let number_station = context.borrow::<EventQueue<i32>>().unwrap();
///
/// number_station.send(123);
///
/// for number in number_station.flush() {
///     // Do something cool.
/// }
/// ```
/// You can instead use the new [Context] methods directly:
///
/// ```
/// # use wolf_engine_core::*;
/// # use wolf_engine_core::events::*;
/// #
/// # let mut context = Context::new();
/// # let event_queue = EventQueue::<i32>::new();
/// # context.add(event_queue);
/// #
/// context.send_event(123);
///
/// for number in context.flush_events::<i32>() {
///     // Do something cool.
/// }
/// ```
///
/// While this trait is intended to extend the [Context], it may be used to extend any type which
/// needs to interact with an [EventQueue].
pub trait EventControls {
    /// Send an event through an [EventQueue] similar to [EventQueue::send()].
    ///
    /// # Panics
    ///
    /// This method will panic if there is no [EventQueue] of type `E`.  If you want to avoid
    /// a panic, you should use [EventControls::try_send_event()] instead.
    fn send_event<E: 'static>(&self, event: E);

    /// Send an event through an [EventQueue] similar to [EventQueue::send()].
    ///
    /// This method will return an [NoEventQueueError] instead of panicking if there is no
    /// [EventQueue] of type `E`.
    fn try_send_event<E: 'static>(&self, event: E) -> Result<(), NoEventQueueError>;

    /// Clear all events off an [EventQueue] and return them similar to [EventQueue::flush()].
    ///
    /// # Panics
    ///
    /// This method will panic if there is no [EventQueue] of type `E`.  If you want to avoid
    /// a panic, you should use [EventControls::try_flush_events()] instead.
    fn flush_events<E: 'static>(&self) -> Vec<E>;

    /// Clear all events off an [EventQueue] and return them similar to [EventQueue::flush()].
    ///
    /// This method will return an [NoEventQueueError] instead of panicking if there is no
    /// [EventQueue] of type `E`.
    fn try_flush_events<E: 'static>(&self) -> Result<Vec<E>, NoEventQueueError>;

    /// Access a [Sender] for a specific [EventQueue] similar to [EventQueue::sender()].
    ///
    /// If there is no [EventQueue] of type `E`, a [None] is returned.
    fn event_sender<E: 'static>(&self) -> Option<Sender<E>>;
}

/// Provides an error indicating there was no [EventQueue] of a requested type.
#[derive(Debug)]
pub struct NoEventQueueError;

