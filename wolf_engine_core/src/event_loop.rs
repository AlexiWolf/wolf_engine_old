use std::sync::Arc;

use crate::events::*;

/// Provides a way to retrieve events from the [`Context`](crate::Context).
///
/// Under the hood, Wolf Engine consists of two main parts: The `EventLoop` (You are here!), and the
/// [`Context`](crate::Context`).  Together, these two parts make up what we refer to as
/// "the engine."
///
/// The Event-Loop is a specialized type of [`EventQueue`].  Unlike a typical Event-Queue, the
/// Event-Loop will continually emit events for as long as the engine is running, even if there
/// are no events currently in the queue.  
///
/// When there are no queued events to emit, [`Event::EventsCleared`] is returned instead, so long
/// as the engine is running.  When [`Event::Quit`] is received, the Event-Loop will trigger a
/// shutdown of the engine.  Only after a shutdown, will the Event-Loop stop emitting events.
///
/// # Examples
///
/// Events are queried using the [`EventQueue` API](crate::events::EventQueue) .
///
/// ```
/// # use wolf_engine_core as wolf_engine;
/// # use wolf_engine::prelude::*;
/// #
/// # let (mut event_loop, mut context) = wolf_engine::init::<()>().build();
/// #
/// while let Some(event) = event_loop.next_event() {
///     match event {
///         // Process events.
/// #       _ => (),
///     }
/// #   break;
/// }
/// ```
pub struct EventLoop<E: UserEvent> {
    event_queue: MpscEventQueue<Event<E>>,
    has_quit: bool,
}

impl<E: UserEvent> EventLoop<E> {
    pub(crate) fn new() -> Self {
        let event_queue = MpscEventQueue::new();
        Self {
            event_queue,
            has_quit: false,
        }
    }

    fn handle_event(&mut self, event: Event<E>) -> Event<E> {
        if event == Event::Quit {
            self.has_quit = true;
        }
        event
    }

    fn handle_empty_event(&self) -> Option<Event<E>> {
        if self.has_quit {
            None
        } else {
            Some(Event::EventsCleared)
        }
    }
}

impl<E: UserEvent> EventQueue<Event<E>> for EventLoop<E> {
    fn next_event(&mut self) -> Option<Event<E>> {
        match self.event_queue.next_event() {
            Some(event) => Some(self.handle_event(event)),
            None => self.handle_empty_event(),
        }
    }
}

impl<E: UserEvent> HasEventSender<Event<E>> for EventLoop<E> {
    fn event_sender(&self) -> Arc<dyn EventSender<Event<E>>> {
        self.event_queue.event_sender()
    }
}

#[cfg(test)]
mod event_loop_tests {
    use ntest::timeout;

    use crate::prelude::*;

    struct TestData {
        updates: i32,
    }

    impl TestData {
        pub fn new() -> Self {
            Self { updates: 0 }
        }
    }

    #[test]
    #[timeout(100)]
    fn should_run_and_quit() {
        let (mut event_loop, mut context) = crate::init::<()>().build();
        let mut updates = 0;

        while let Some(event) = event_loop.next_event() {
            process_event(event, &mut context, &mut updates);
        }

        assert!(event_loop.has_quit);
        assert_eq!(updates, 3);
    }

    fn process_event<E: UserEvent>(event: Event<E>, context: &mut Context<E>, updates: &mut i32) {
        match event {
            Event::Quit => (),
            Event::EventsCleared => {
                if *updates == 3 {
                    context.quit();
                } else {
                    *updates += 1;
                }
            }
            _ => (),
        }
    }
}

#[test]
fn should_emit_events_cleared_when_event_queue_is_empty() {
    let (mut event_loop, context) = crate::init::<()>().build();

    context
        .event_sender()
        .send_event(Event::UserDefined(()))
        .ok();
    assert_eq!(
        event_loop.next_event().unwrap(),
        Event::UserDefined(()),
        "The event-loop did not emit the expected Test event."
    );

    assert_eq!(
        event_loop.next_event().unwrap(),
        Event::EventsCleared,
        "The event-loop did not emit the expected EventsCleared event."
    );
}
