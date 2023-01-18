use std::sync::Arc;

use crate::events::*;
use crate::prelude::*;

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
}

impl Engine<(), EventQueue<Event>> {
    pub fn new() -> Self {
        let event_loop = EventQueue::new();
        Self {
            context: Context::new(&event_loop, ()),
            event_loop,
        }
    }
}

impl Default for Engine<(), EventQueue<Event>> {
    fn default() -> Self {
        Self::new()
    }
}

impl<D> From<D> for Engine<D, EventQueue<Event>> {
    fn from(data: D) -> Self {
        let event_loop = EventQueue::new();
        Self {
            context: Context::new(&event_loop, data),
            event_loop,
        }
    }
}

impl<D, E: EventLoop<Event>> EngineControls for Engine<D, E> {
    fn quit(&self) {
        self.context.quit()
    }

    fn has_quit(&self) -> bool {
        self.context.has_quit()
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
            self.context.set_has_quit(true);
        }
        event
    }

    fn handle_empty_event(&self) -> Option<Event> {
        if self.context.has_quit() {
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

    fn sender(&self) -> Arc<dyn EventSender<Event>> {
        self.event_loop.sender()
    }
}

#[cfg(test)]
mod engine_tests {
    use ntest::timeout;

    use super::*;
    use crate::events::*;

    struct TestData {
        message: String,
        updates: i32,
        renders: i32,
    }

    impl TestData {
        pub fn new() -> Self {
            Self {
                message: "Hello, World!".to_string(),
                updates: 0,
                renders: 0,
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

        while let Some(event) = engine.next_event() {
            process_event(event, engine.context_mut());
        }

        assert!(engine.has_quit());
        assert_eq!(engine.context().data.updates, 3);
        assert_eq!(engine.context().data.renders, 4);
    }

    fn process_event(event: Event, context: &mut Context<TestData>) {
        match event {
            Event::Quit => (),
            Event::Update => {
                if context.data.updates < 3 && context.data.renders < 3 {
                    context.data.updates += 1;
                } else {
                    context.quit();
                }
            }
            Event::Render => context.data.renders += 1,
            Event::EventsCleared => {
                context.update();
                context.render();
            }
            _ => (),
        }
    }

    #[test]
    fn should_emit_events_cleared_when_event_queue_is_empty() {
        let mut engine = Engine::from(TestData::new());

        assert_eq!(engine.next_event().unwrap(), Event::EventsCleared);
    }
}
