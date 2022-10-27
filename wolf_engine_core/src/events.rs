//! Provides an event system for the engine.

use std::sync::mpsc::{channel, Receiver, Sender};

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

/// Provides a generic, fifo, mpsc event queue based on [std::sync::mpsc].
///
/// # Examples
///
/// To create an `EventQueue`, use [EventQueue::new()].  You must specify the event type you wish
/// to use, or allow Rust to figure it out based on usage.
///
/// ```
/// # use wolf_engine_core::events::EventQueue;
/// #
/// # enum EventType { Event };
/// #
/// let event_queue = EventQueue::<EventType>::new();
/// ```
///
/// Events can be sent directly through [EventQueue::send()] if you have direct access to the
/// `EventQueue`.
///
/// ```
/// # use wolf_engine_core::events::EventQueue;
/// #
/// # enum EventType { Event };
/// #
/// let event_queue = EventQueue::new();
/// event_queue.send(EventType::Event);
/// ```
///
/// The `EventQueue` itself cannot be sent across threads, so if you need to send events across
/// threads, you must create a [Sender] using [EventQueue::sender()].  A [Sender] can also be used
/// to send events from code which does not have direct access to the `EventQueue`.
///
/// ```
/// # use wolf_engine_core::events::EventQueue;
/// #
/// # enum EventType { Event };
/// #
/// let event_queue = EventQueue::new();
/// let event_sender = event_queue.sender();
///
/// std::thread::spawn(move || {
///     event_sender.send(EventType::Event).unwrap();
/// })
/// # .join()
/// # .unwrap();
/// ```
///
/// Queued events can be accessed by calling [EventQueue::flush()] this will clear all events from
/// the queue and return them in a collection which can be iterated over.
///
/// ```
/// # use wolf_engine_core::events::EventQueue;
/// #
/// # enum EventType { Event };
/// #
/// # let event_queue = EventQueue::<i32>::new();
/// #
/// for event in event_queue.flush() {
///     // Handle events here.
/// }
/// ```
pub struct EventQueue<E> {
    sender: Sender<E>,
    receiver: Receiver<E>,
}

impl<E> EventQueue<E> {
    /// Creates a new event queue.
    pub fn new() -> Self {
        let (sender, receiver) = channel();
        Self { sender, receiver }
    }

    /// Send an event to the event queue.
    pub fn send(&self, event: E) {
        self.sender.send(event).unwrap();
    }

    /// Creates a new [Sender] from the event queue.
    ///
    /// A [Sender] can be created and moved to code to send events across threads, or to send
    /// events without direct access to the event queue.
    pub fn sender(&self) -> Sender<E> {
        self.sender.clone()
    }

    /// Clears all events off the queue and returns them in a collection which can be iterated over.
    pub fn flush(&self) -> Vec<E> {
        self.receiver.try_iter().collect()
    }
}

impl<E> EventLoop<E> for EventQueue<E> {
    fn next_event(&self) -> Option<E> {
        self.receiver.try_recv().ok()
    }

    fn send_event(&self, event: E) {
        self.send(event)
    }
}

impl<E> Default for EventQueue<E> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod event_queue_tests {
    use std::thread;

    pub use super::*;

    #[test]
    pub fn should_send_and_receive_events() {
        let event_queue = EventQueue::new();

        event_queue.send(0);
        let events = event_queue.flush();

        assert_eq!(events.get(0).expect("No event in the queue"), &0);
    }

    #[test]
    pub fn should_send_events_through_a_sender() {
        let event_queue = EventQueue::new();
        let sender = event_queue.sender();

        sender.send(0).unwrap();
        thread::spawn(move || {
            sender.send(1).unwrap();
        })
        .join()
        .unwrap();

        let events = event_queue.flush();
        assert_eq!(events.get(0).unwrap(), &0);
        assert_eq!(events.get(1).unwrap(), &1);
    }

    #[test]
    pub fn should_flush_empty_list_if_there_are_no_events() {
        let event_queue = EventQueue::<i32>::new();

        assert!(event_queue.flush().is_empty());
    }

    #[test]
    pub fn should_implement_default_trait() {
        let _event_queue = EventQueue::<i32>::default();
    }
}
