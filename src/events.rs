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
/// # use wolf_engine::*;
/// # use wolf_engine::events::*;
/// #
/// # let mut context = Context::new();
/// # let event_queue = EventQueue::<i32>::new();
/// # context.add(event_queue);
/// #
/// let number_station = context.borrow::<EventQueue<i32>>().unwrap();
/// 
/// number_station.send_event(123);
///
/// for number in number_station.flush() {
///     // Do something cool.
/// }
/// ```
/// You can instead use the new [Context] methods directly:
///
/// ```
/// # use wolf_engine::*;
/// # use wolf_engine::events::*;
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
pub trait EventControls {
    fn send_event<E>(event: E);
    fn flush_events<E>() -> Vec<E>;
}

/// Provides a generic fifo, mpsc event queue based on [std::sync::mpsc].
///
/// # Examples
///
/// To create an `EventQueue`, use [EventQueue::new()].  You must specify the event type you wish
/// to use, or allow Rust to figure it out based on usage.
///
/// ```
/// # use wolf_engine::events::EventQueue;
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
/// # use wolf_engine::events::EventQueue;
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
/// # use wolf_engine::events::EventQueue;
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
/// # use wolf_engine::events::EventQueue;
/// #
/// # enum EventType { Event };
/// #
/// # let event_queue = EventQueue::<i32>::new();
/// #
/// for event in event_queue.flush() {
///     // Handle events here.
/// }
/// ```
///
/// # [Context] Integrations
///
/// The `EventQueue` is designed to be easily used used with the [Context].  First, it is marked 
/// as a [Subcontext], allowing you to attach an `EventQueue` directly to the [Context] object.
/// 
/// ```
/// # use wolf_engine::*;
/// # use wolf_engine::events::*;
/// #
/// # let mut context = Context::new();
/// #
/// let number_station = EventQueue::<i32>::new();
/// context.add(number_station);
/// 
/// let _number_station = context.borrow::<EventQueue<i32>>().unwrap();
/// ```
///
/// Second, the [EventControls] trait provides [EventQueue] methods usable directly on the 
/// [Context].
///
/// Todo: Add usage examples.
///
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

impl<E> Default for EventQueue<E> {
    fn default() -> Self {
        Self::new()
    }
}

impl<E: 'static> Subcontext for EventQueue<E> {}

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
