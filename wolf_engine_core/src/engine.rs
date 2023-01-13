use crate::events::{Event, EventLoop, EventQueue};
use crate::{Context, EngineControls};

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
/// 3. If the [`Context`] returns [`None`], and [`Event::Quit`] has not been received, return
///    [`Event::EventsCleared`].
/// 4. If the [`Context`] returns [`None`] and [`Event::Quit`] has been received, return [`None`].
///
/// # Examples
///
/// ```
/// # use wolf_engine_core::*;
/// # use wolf_engine_core::events::*;
/// #
/// let mut engine = Engine::new();
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
///             engine.quit();
///         },
///         Event::Render => {
///             // Render the game.
///         },
///         Event::EventsCleared => {
///             // Note: The engine will not emit Update / Render events on its own.
///             //       You are expected to do this yourself.
///             engine.update();
///             engine.render();
///         }
///         _ => (),
///     }
/// }
/// ```
pub struct Engine<D, E: EventLoop<Event>> {
    context: Context<D>,
    event_loop: E,
    has_quit: bool,
}

impl<D> Engine<D, EventQueue<Event>> {
    pub fn new() -> Self {
        Self {
            context: Context::from(()), 
            event_loop: EventQueue::new(),
            has_quit: false,
        }
    }
}

impl Default for Engine<(), EventQueue<Event>> {
    fn default() -> Self {
        Self::new()
    }
}

impl<D> From<D> for Engine<D, EventQueue<Event>> {
    fn from(context: Context<D>) -> Self {
        Self {
            context,
            event_loop: EventQueue::new(),
            has_quit: false,
        }
    }
}

impl<D, E: EventLoop<Event>> EngineControls for Engine<D, E> {
    fn quit(&self) {
        self.send_event(Event::Quit);
    }

    fn has_quit(&self) -> bool {
        self.has_quit
    }

    fn update(&self) {
        self.send_event(Event::Update);
    }

    fn render(&self) {
        self.send_event(Event::Render);
    }
}

impl<D, E: EventLoop<Event>> Engine<D, E> {
    /// Get immutable access to the [`Context`] data.
    pub fn context(&self) -> &Context<D> {
        &self.context
    }

    /// Get mutable access to the [`Context`] data.
    pub fn context_mut(&mut self) -> &mut Context<D> {
        &mut self.context
    }

    fn handle_event(&mut self, event: Event) -> Event {
        if event == Event::Quit {
            self.has_quit = true;
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

impl<D, E: EventLoop<Event>> EventLoop<Event> for Engine<D, E> {
    fn next_event(&mut self) -> Option<Event> {
        match self.event_loop.next_event() {
            Some(event) => Some(self.handle_event(event)),
            None => self.handle_empty_event(),
        }
    }

    fn send_event(&self, event: Event) {
        self.event_loop.send_event(event)
    }
}

#[cfg(test)]
mod engine_tests {
    use ntest::timeout;

    use super::*;
    use crate::events::*;

    struct TestData {
        message: String,
    }

    impl TestData {
        pub fn new() -> Self {
            Self {
                message: "Hello, World!".to_string(),
            }
        }
    }

    #[test]
    fn should_provide_context_accessors() {
        let mut engine = Engine::from(TestData::new());

        assert_eq!(engine.context().data.message, "Hello, World!");
        engine.context_mut().data.message = "New message!".to_string();
        assert_eq!(engine.context().data.message, "New message!");
    }

    #[test]
    #[timeout(100)]
    fn should_run_and_quit() {
        let mut engine = Engine::from(TestData::new());
        let mut updates = 0;
        let mut renders = 0;

        while let Some(event) = engine.next_event() {
            match event {
                Event::Quit => (),
                Event::Update => {
                    if updates < 3 && renders < 3 {
                        updates += 1;
                    } else {
                        engine.quit();
                    }
                }
                Event::Render => renders += 1,
                Event::EventsCleared => {
                    engine.update();
                    engine.render();
                }
                _ => (),
            }
        }

        assert!(engine.has_quit());
    }

    #[test]
    fn should_emit_events_cleared_when_event_queue_is_empty() {
        let mut engine = Engine::from(TestData::new());

        assert_eq!(engine.next_event().unwrap(), Event::EventsCleared);
    }
}
