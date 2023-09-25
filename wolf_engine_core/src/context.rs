use std::sync::Arc;

use crate::events::*;
use crate::ecs::*;

/// Provides a container for Wolf Engine's user-facing data.
///
/// Under the hood, Wolf Engine consists of two main parts: The `Context` (You are here!), and the
/// [`EventLoop`](crate::EventLoop`).  Together, these two parts make up what we refer to as
/// "the engine."
///
/// The Context owns all engine data, sub-systems, and the link to the Event-Loop through which
/// all events are sent.  As far as the end-user is concerned, the Context *is* the engine.
pub struct Context<E: UserEvent> {
    event_sender: Arc<dyn EventSender<Event<E>>>,
}

impl<E: UserEvent> Context<E> {
    /// Create a new `Context` from the provided [`EventQueue`] and data.
    pub(crate) fn new(event_queue: &dyn EventQueue<Event<E>>) -> Self {
        Self {
            event_sender: event_queue.event_sender(),
        }
    }

    pub fn add_resource<T>(&mut self, resource: T) {
        
    }

    pub fn resource<T>(&self) -> Option<T> {
        None
    }

    pub fn quit(&self) {
        self.event_sender.send_event(Event::Quit).ok();
    }
}

impl<E: UserEvent> HasEventSender<Event<E>> for Context<E> {
    fn event_sender(&self) -> Arc<dyn EventSender<Event<E>>> {
        self.event_sender.clone()
    }
}

#[cfg(test)]
mod context_tests {
    use crate::EventLoop;

    use super::*;

    #[test]
    fn should_add_and_access_resources() {
        struct TestResource(&'static str);
        let event_loop = EventLoop::<()>::new();
        let mut context = Context::new(&event_loop);

        context.add_resource(TestResource("Hello, World!"));
        let resource = context.resource::<TestResource>().expect("Resource doesn't exist");
    }
}
