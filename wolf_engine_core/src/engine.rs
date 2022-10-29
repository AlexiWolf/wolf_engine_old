use crate::events::{Event, EventLoop};
use crate::Context;

/// Provides a wrapper around some [`Context`] data with [`EventLoop`] and quit behavior.
///
/// The `Engine` is a small wrapper around the [`Context`] data providing a few useful utilities 
/// such as an [`EventLoop`] implementation with properly-handled quit behaviors.  The provided
/// [`EventLoop`] implementation is better suited to how the engine is intended to be used The 
/// engine is generic over the [`Context`] data, allowing users to easily extend and modify the 
/// engine's capabilities while keeping a consistent interface.  
///
/// The [`EventLoop`] algorithm:
///
/// 1. Get the latest queued [`Event`] from the [`Context`].
/// 2. If the [`Context`] returns [`Some`], return the [`Event`].
/// 3. If the [`Context`] returns [`None`], and [`Event::Quit`] has not been recived, return 
///    [`Event::EventsCleared`].
/// 4. If the [`Context`] returns [`None`] and [`Event::Quit`] has been recieved, return [`None`].
///
/// # Examples
///
/// ```
/// # use wolf_engine_core::*;
/// # use wolf_engine_core::events::*;
/// #
/// // The Engine requires Context data to be created, in this case, we're just using an
/// // EventQueue.  
/// let mut engine = Engine::new(EventQueue::new());
///
/// // The Engine will continue to return events until it quits.
/// while let Some(event) = engine.next_event() {
///     match event {
///         Event::Quit => {
///             // Shut down the game.
///         },
///         Event::Update => {
///             // Update the game.
///
///             // To shut down the Engine, you must send a quit event.
///             engine.send_event(Event::Quit);
///         },
///         Event::Render => {
///             // Render the game.
///         },
///         Event::EventsCleared => {
///             // Note: The engine will not emit Update / Render events on its own.
///             //       You are expected to do this yourself.
///             engine.send_event(Event::Update);
///             engine.send_event(Event::Render);
///         }
///     }
/// }
/// ```
pub struct Engine<C: Context<Event>> {
    context: C,
    has_quit: bool,
}

impl<C: Context<Event>> Engine<C> {
    pub fn new(context: C) -> Self {
        Self {
            context,
            has_quit: false,
        }
    }

    pub fn has_quit(&self) -> bool {
        self.has_quit
    }

    pub fn context(&self) -> &C {
        &self.context
    }

    pub fn context_mut(&mut self) -> &mut C {
        &mut self.context
    }

    fn handle_event(&mut self, event: Event) -> Event {
        match event {
            Event::Quit => self.has_quit = true,
            _ => (),
        }
        event
    }

    fn handle_empty_event(&self) -> Option<Event> {
        if self.has_quit {
            None
        } else {
            Some(Event::EventsCleared)
        }
    }
}

impl<C: Context<Event>> EventLoop<Event> for Engine<C> {
    fn next_event(&mut self) -> Option<Event> {
        match self.context.next_event() {
            Some(event) => Some(self.handle_event(event)),
            None => self.handle_empty_event(),
        }
    }

    fn send_event(&self, event: Event) {
        self.context.send_event(event)
    }
}

#[cfg(test)]
mod engine_tests {
    use ntest::timeout;

    use super::*;
    use crate::events::*;

    struct TestData {
        message: String,
        event_queue: EventQueue<Event>,
    }

    impl TestData {
        pub fn new() -> Self {
            Self {
                message: "Hello, World!".to_string(),
                event_queue: EventQueue::new(),
            }
        }
    }

    impl EventLoop<Event> for TestData {
        fn next_event(&mut self) -> Option<Event> {
            self.event_queue.next_event()
        }

        fn send_event(&self, event: Event) {
            self.event_queue.send_event(event)
        }
    }

    #[test]
    fn should_provide_context_accessors() {
        let mut engine = Engine::new(TestData::new());

        assert_eq!(engine.context().message, "Hello, World!");
        engine.context_mut().message = "New message!".to_string();
        assert_eq!(engine.context().message, "New message!");
    }

    #[test]
    #[timeout(100)]
    fn should_run_and_quit() {
        let mut engine = Engine::new(TestData::new());
        let mut number = 0;

        while let Some(event) = engine.next_event() {
            match event {
                Event::Quit => (),
                Event::Update => {
                    if number < 3 {
                        number += 1;
                    } else {
                        engine.send_event(Event::Quit);
                    }
                }
                Event::Render => (),
                Event::EventsCleared => {
                    engine.send_event(Event::Quit);
                }
            }
        }

        assert!(engine.has_quit());
    }

    #[test]
    fn should_emit_events_cleared_when_event_queue_is_empty() {
        let mut engine = Engine::new(TestData::new());

        assert_eq!(engine.next_event().unwrap(), Event::EventsCleared);
    }
}
