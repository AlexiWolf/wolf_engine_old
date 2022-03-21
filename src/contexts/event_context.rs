use std::{pin::Pin, sync::Arc};

use crate::{
    event::{EventQueue, EventReader},
    Subcontext,
};

/// Provides an event system usable through the [Context](crate::Context).
///
/// The Event Context is a small wrapper around [rc_event_queue]s API.  It helps to 
/// integrate everything more cleanly into the engine and is a bit more friendly than 
/// using [EventQueue] and [EventReader] directly.
///
/// # Examples
///
/// To create a new Event Context, you should use [EventContext::default()].  You must 
/// provide an event type parameter, or a type annotation to indicate which type you want
/// to use an event.  Most types can be used as an Event.
///
/// ```
/// # use wolf_engine::contexts::EventContext;
/// # use wolf_engine::event::Event; 
/// #
/// let event_context = EventContext::<Event>::default();
/// ```
/// Or, alternatively:
///
/// ```
/// # use wolf_engine::contexts::EventContext;
/// # use wolf_engine::event::Event; 
/// #
/// let event_context: EventContext<Event> = EventContext::default();
/// ```
/// 
/// Pushing events is done using [EventContext::push()] like so:
///
/// ```
/// # use wolf_engine::contexts::EventContext;
/// # use wolf_engine::event::Event; 
/// #
/// # let event_context = EventContext::<usize>::default();
/// # let event = 0;
/// #
/// event_context.push(event);
/// ```
///
/// Processing events is done through an [EventReader].  You can get an [EventReader]
/// using [EventContext::reader()].
///
/// ```
/// # use wolf_engine::contexts::EventContext;
/// # use wolf_engine::event::Event; 
/// #
/// # let event_context = EventContext::<usize>::default();
/// #
/// let mut event_reader = event_context.reader();
/// ```
///
/// ```
/// # use wolf_engine::contexts::EventContext;
/// # use wolf_engine::event::*;
/// #
/// # let event_context = EventContext::<usize>::default();
/// # let mut event_reader = event_context.reader();
/// #
/// while let Some(event) = event_reader.iter().next() {
///     // Process the event.
/// }
/// ```
///
/// In most cases, you're not going to be using an Event Context in isolation.  You'll
/// probably be working with an existing Event Context stored on the 
/// [Context](crate::Context).  You can access specific Event Contexts through the 
/// [Context::get()](crate::Context) method.
///
/// **Note:** There are no methods on the Event Context that require a mutable reference, 
/// so make sure you use [Context::get()](crate::Context) for an immutable reference.
///
/// ```
/// # use wolf_engine::Context;
/// # use wolf_engine::contexts::EventContext;
/// # use wolf_engine::event::Event; 
/// #
/// # let mut context = Context::empty();
/// # context.add(EventContext::<Event>::default()).unwrap();
/// #
/// let event_context = context.get::<EventContext<Event>>().expect("no event context");
/// ```
///
/// # Preventing Memory Leaks
///
/// One of the main problems with using [EventQueue] and [EventReader] directly, is it's 
/// possible to trigger memory leaks if an [EventReader] is created, but not read from.
/// The [EventQueue] does not drop events unless all of its [EventReader]s have read the
/// event.  This allows a rogue [EventReader] to cause events to stack up, resulting in 
/// a memory leak.
///
/// The Event Context helps to prevent this issue by imposing a limit on how many events
/// the [EventQueue] can contain.  If this limit is exceeded, the oldest events will be 
/// forcibly removed and dropped.  While this means some events may never be processed, 
/// in most cases this is preferable to crashing.
///
/// The best way to ensure the event cap is not reached is to ensure all [EventReader]s 
/// are being read from, and are dropped when you are done with them.
///
/// ## Increasing the Queue Size Limit 
///
/// The default event cap is about 100K, and in most cases (assuming your game is working
/// normally), you shouldn't get anywhere near the limit.  If you find the cap isn't 
/// enough for your game, the cap can be increased using [EventContext::new()].
///
/// ```Rust
/// # use wolf_engine::contexts::EventContext;
/// # use wolf_engine::event::Event;
/// #
/// # let custom_queue_limit = 20_000;
/// #
/// let event_context = EventContext::<Event>::new(custom_queue_limit);
/// ```
pub struct EventContext<E> {
    event_queue: Pin<Arc<EventQueue<E>>>,
    pub max_queue_size: usize,
}

impl<E> EventContext<E> {
    pub fn new(max_queue_size: usize) -> Self {
        Self {
            event_queue: EventQueue::new(),
            max_queue_size,
        }
    }

    pub fn push(&self, event: E) {
        self.event_queue.push(event);
        self.truncate_queue_if_over_max_capacity();
    }

    fn truncate_queue_if_over_max_capacity(&self) {
        if self.event_queue.total_capacity() > self.max_queue_size {
            self.event_queue.truncate_front(1024);
        }
    }

    pub fn reader(&self) -> EventReader<E> {
        EventReader::new(&self.event_queue)
    }
}

impl<E> Default for EventContext<E> {
    fn default() -> Self {
        Self {
            event_queue: EventQueue::new(),
            max_queue_size: 100_000,
        }
    }
}

impl<E: 'static> Subcontext for EventContext<E> {}

#[cfg(test)]
mod event_context_tests {
    use std::fmt::Debug;

    use crate::event::LendingIterator;

    pub use super::*;

    #[test]
    fn should_push_events_to_queue() {
        let events = EventContext::<u32>::default();
        let mut reader = events.reader();

        events.push(1);

        assert_next_event_equals(&mut reader, &1);
    }

    fn assert_next_event_equals<E: Eq + PartialEq + Debug>(
        reader: &mut EventReader<E>,
        expected: &E,
    ) {
        while let Some(event) = reader.iter().next() {
            assert_eq!(event, expected, "the events do not match");
            return;
        }
        panic!("No events received")
    }

    #[test]
    fn should_prevent_memory_leaks_by_dropping_events_from_the_queue() {
        let events = EventContext::<u32>::new(20_472);
        let _unused_reader = events.reader();

        for i in 0..=50_000 {
            events.push(i);
        }

        let queue_size = events.event_queue.total_capacity();
        assert!(
            queue_size <= 20_472,
            "Expected capacity of up to 20,472, but got {} events",
            queue_size
        );
    }

    #[test]
    fn should_not_drop_events_unless_over_max_queue_size() {
        let events = EventContext::<u32>::new(20_472);
        let _unused_reader = events.reader();

        for i in 0..20_472 {
            events.push(i);
        }

        let queue_size = events.event_queue.total_capacity();
        assert!(
            queue_size == 20_472,
            "Expected capacity of 20,472 events, but got {}",
            queue_size
        );
    }
}
