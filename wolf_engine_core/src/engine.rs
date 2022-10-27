use crate::events::*;

#[derive(Debug, PartialEq, Eq)]
pub enum Event {
    Quit,
    Update,
    Render,
    EventsCleared,
}

pub trait Context<E>: EventLoop<E> {}

pub trait EventLoop<E> {
    fn next_event(&self) -> Option<E>;
    fn send_event(&self, event: E);
}

pub struct Engine<C: Context<Event>> {
    context: C,
}

impl<C: Context<Event>> Engine<C> {
    pub fn new(context: C) -> Self {
        Self { context }
    }

    pub fn context(&self) -> &C {
        &self.context
    }

    pub fn context_mut(&mut self) -> &mut C {
        &mut self.context
    }
}

impl<C: Context<Event>> EventLoop<Event> for Engine<C> {
    fn next_event(&self) -> Option<Event> {
        None
    }

    fn send_event(&self, event: Event) {}
}

#[cfg(test)]
mod engine_tests {
    use super::*;

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

    impl Context<Event> for TestData {}

    impl EventLoop<Event> for TestData {
        fn next_event(&self) -> Option<Event> {
            self.event_queue.next_event()
        }

        fn send_event(&self, event: Event) {
            self.event_queue.send(event)
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
    fn should_take_events() {
        let engine = Engine::new(TestData::new());

        while let Some(event) = engine.next_event() {
            match event {
                Event::Quit => (),
                Event::Update => (),
                Event::Render => (),
                Event::EventsCleared => {
                    engine.send_event(Event::Quit);
                },
            }
        }
    }

    #[test]
    fn should_emit_events_cleared_when_event_queue_is_empty() {
        let mut engine = Engine::new(TestData::new()); 

        assert_eq!(engine.next_event().unwrap(), Event::EventsCleared);
    }
}
