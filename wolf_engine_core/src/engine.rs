use std::sync::Arc;

use crate::events::EventSender;
use crate::events::*;
use crate::prelude::*;

pub struct EventLoop {
    event_queue: MpscEventQueue<Event>,
    has_quit: bool,
}

impl EventLoop {
    pub fn new<D>(data: D) -> (Self, Context<D>) {
        let event_queue = MpscEventQueue::new();
        let event_loop = Self { event_queue, has_quit: false };
        let context = Context::new(&event_loop, data);
        (event_loop, context)
    }
}

impl EventLoop {
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

impl EventQueue<Event> for EventLoop {
    fn next_event(&mut self) -> Option<Event> {
        match self.event_queue.next_event() {
            Some(event) => Some(self.handle_event(event)),
            None => self.handle_empty_event(),
        }
    }
}

impl HasEventSender<Event> for EventLoop {
    fn event_sender(&self) -> Arc<dyn EventSender<Event>> {
        self.event_queue.event_sender()
    }
}

#[cfg(test)]
mod event_loop_tests {
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
        let (mut event_loop, mut context) = EventLoop::new(TestData::new());

        assert_eq!(context.data.message, "Hello, World!");
        context.data.message = "New message!".to_string();
        assert_eq!(context.data.message, "New message!");
    }

    #[test]
    #[timeout(100)]
    fn should_run_and_quit() {
        let (mut event_loop, mut context) = EventLoop::new(TestData::new());

        while let Some(event) = event_loop.next_event() {
            process_event(event, &mut context);
        }

        assert!(event_loop.has_quit);
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
        let (mut event_loop, _context) = EventLoop::new(TestData::new());

        assert_eq!(event_loop.next_event().unwrap(), Event::EventsCleared);
    }
}
