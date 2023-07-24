use std::sync::Arc;

use crate::events::EventSender;
use crate::events::*;
use crate::prelude::*;

// TODO: Re-structure the `Engine` type into separate, and more focused
// `EventLoop`, and `Context` types.
pub struct Engine {
    event_loop: MpscEventQueue<Event>,
}

impl Engine {
    pub fn new() -> (Self, Context<()>) {
        let event_loop = MpscEventQueue::new();
        let event_loop = Self { event_loop, };
        let context = Context::new(&event_loop, ());
        (event_loop, context)
    }
}

impl Default for Engine {
    fn default() -> Self {
        Self::new()
    }
}

impl Engine {
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

impl EventQueue<Event> for Engine {
    fn next_event(&mut self) -> Option<Event> {
        match self.event_loop.next_event() {
            Some(event) => Some(self.handle_event(event)),
            None => self.handle_empty_event(),
        }
    }
}

impl HasEventSender<Event> for Engine {
    fn event_sender(&self) -> Arc<dyn EventSender<Event>> {
        self.event_loop.event_sender()
    }
}

#[cfg(test)]
mod engine_tests {
    use ntest::timeout;

    use super::*;

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
        let (mut engine, mut context) = Engine::from(TestData::new());

        assert_eq!(context.data.message, "Hello, World!");
        context.data.message = "New message!".to_string();
        assert_eq!(context.data.message, "New message!");
    }

    #[test]
    #[timeout(100)]
    fn should_run_and_quit() {
        let (mut engine, mut context) = Engine::from(TestData::new());

        while let Some(event) = engine.next_event() {
            process_event(event, &mut context);
        }

        assert!(context.has_quit());
        assert_eq!(context.data.updates, 3);
        assert_eq!(context.data.renders, 4);
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
        let (mut engine, _context) = Engine::from(TestData::new());

        assert_eq!(engine.next_event().unwrap(), Event::EventsCleared);
    }
}
