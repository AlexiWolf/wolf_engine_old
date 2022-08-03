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
/// number_station.send(123);
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
///
/// While this trait is intended to extend the [Context], it may be used to extend any type which
/// needs to interact with an [EventQueue].
pub trait EventControls {
    fn send_event<E: 'static>(&self, event: E);
    fn try_send_event<E: 'static>(&self, event: E) -> Result<(), NoEventQueueError>;
    fn flush_events<E: 'static>(&self) -> Vec<E>;
    fn try_flush_events<E: 'static>(&self) -> Result<Vec<E>, NoEventQueueError>;
    fn event_sender<E: 'static>(&self) -> Option<Sender<E>>;
}

impl EventControls for Context {
    fn send_event<E: 'static>(&self, event: E) {
        let event_queue = self
            .borrow::<EventQueue<E>>()
            .expect("There is no EventQueue of the requested type");
        event_queue.send(event);
    }

    fn try_send_event<E: 'static>(&self, event: E) -> Result<(), NoEventQueueError> {
        if let Some(event_queue) = self.borrow::<EventQueue<E>>() {
            event_queue.send(event);
            Ok(())
        } else {
            Err(NoEventQueueError)
        }
    }

    fn flush_events<E: 'static>(&self) -> Vec<E> {
        let event_queue = self
            .borrow::<EventQueue<E>>()
            .expect("There is no EventQueue of the requested type");
        event_queue.flush()
    }

    fn try_flush_events<E: 'static>(&self) -> Result<Vec<E>, NoEventQueueError> {
        if let Some(event_queue) = self.borrow::<EventQueue<E>>() {
            Ok(event_queue.flush())
        } else {
            Err(NoEventQueueError)
        }
    }
    
    fn event_sender<E: 'static>(&self) -> Option<Sender<E>> {
        if let Some(event_queue) = self.borrow::<EventQueue<E>>() {
            Some(event_queue.sender())
        } else {
            None
        }
    }
}

#[derive(Debug)]
pub struct NoEventQueueError;

#[cfg(test)]
mod event_controls_context_implementation_tests {
    use super::*;

    #[test]
    fn should_send_and_receive_events_through_the_context() {
        let mut context = Context::new();
        context.add(EventQueue::<i32>::new()).unwrap();

        context.send_event(10 as i32);
        let events = context.flush_events::<i32>();
        let number = events
            .get(0)
            .expect("Failed to access the number in the event queue");

        assert_eq!(number, &10);
    }

    #[test]
    #[should_panic(expected = "There is no EventQueue of the requested type")]
    fn should_panic_when_sending_events_if_there_is_no_event_queue() {
        let context = Context::new();
        context.send_event(10);
    }

    #[test]
    #[should_panic(expected = "There is no EventQueue of the requested type")]
    fn should_panic_when_flushing_events_if_there_is_no_event_queue() {
        let context = Context::new();
        let _events = context.flush_events::<i32>();
    }

    #[test]
    fn should_send_and_receive_events_through_try_methods() {
        let mut context = Context::new();
        context.add(EventQueue::<i32>::new()).unwrap();

        context
            .try_send_event(10)
            .expect("Failed to send the event");
        let events = context
            .try_flush_events::<i32>()
            .expect("Failed to flush events");
        let number = events
            .get(0)
            .expect("Failed to access the number in the event queue");

        assert_eq!(number, &10);
    }

    #[test]
    fn should_return_err_from_try_methods_when_there_is_no_event_queue() {
        let context = Context::new();

        let send_result = context.try_send_event(10);
        let flush_result = context.try_flush_events::<i32>();

        assert!(
            send_result.is_err(),
            "Expected a NoEventQueueError, but was Ok"
        );
        assert!(
            flush_result.is_err(),
            "Expected a NoEventQueueError, but was Ok"
        );
    }
}

/// Provides a generic, fifo, mpsc event queue based on [std::sync::mpsc].
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
/// The `EventQueue` is designed to be easily used with the [Context].  First, it is marked
/// as a [Subcontext], allowing you to attach an `EventQueue` directly to the [Context] object.  
///
/// You can add, then access the `EventQueue` same as any other [Subcontext]:
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
/// The [EventControls] trait provides `EventQueue` methods usable directly on the [Context].  
/// These methods use the data type (`E`) you provide to figure out which `EventQueue` to use.
/// For example:
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
///
/// **Note:** Because [EventControls::send_event()], and [EventControls::flush_events()] will panic
/// if an `EventQueue` of type `E` is not present, you may want to use 
/// [EventControls::try_send_event()] and [EventControls::try_flush_events()] instead.
///
/// ```
/// # use wolf_engine::*;
/// # use wolf_engine::events::*;
/// #
/// # let mut context = Context::new();
/// # let event_queue = EventQueue::<i32>::new();
/// # context.add(event_queue);
/// #
/// context.try_send_event(123).unwrap();
///
/// if let Ok(events) = context.try_flush_events::<i32>() {
///     for event in events {
///         // Do something cool.
///     }
/// }
/// ```
/// It's also possible to get a [Sender] using [EventControls::event_sender()]:
/// 
/// ```
/// # use wolf_engine::*;
/// # use wolf_engine::events::*;
/// #
/// # let mut context = Context::new();
/// # let event_queue = EventQueue::<i32>::new();
/// # context.add(event_queue);
/// #
/// let sender = context.event_sender::<i32>().unwrap();
/// sender.send(10).unwrap();
///
/// if let Ok(events) = context.try_flush_events::<i32>() {
///     for event in events {
///         // Do something cool.
///     }
/// }
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
