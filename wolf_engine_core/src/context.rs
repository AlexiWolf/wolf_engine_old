use std::sync::Arc;

use atomic_refcell::AtomicRef;
use atomic_refcell::AtomicRefMut;

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
    resources: Resources,
    event_sender: Arc<dyn EventSender<Event<E>>>,
}

impl<E: UserEvent> Context<E> {
    /// Create a new `Context` from the provided [`EventQueue`] and data.
    pub(crate) fn new(event_queue: &dyn EventQueue<Event<E>>) -> Self {
        Self {
            resources: Resources::default(),
            event_sender: event_queue.event_sender(),
        }
    }

    pub fn add_resource<T: 'static>(&mut self, resource: T) {
        self.resources.insert(resource);  
    }

    pub fn resource<T: 'static>(&self) -> Option<AtomicRef<T>> {
        self.resources.get::<T>()
    }

    pub fn resource_mut<T: 'static>(&mut self) -> Option<AtomicRefMut<T>> {
        self.resources.get_mut()
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
        
    struct TestResource(&'static str);

    pub fn test_init() -> (EventLoop<()>, Context<()>) {
        let event_loop = EventLoop::<()>::new();
        let mut context = Context::new(&event_loop);
        (event_loop, context)
    }

    #[test]
    fn should_add_and_access_resources() {
        let (_, mut context) = test_init();

        context.add_resource(TestResource("Hello, World!"));
        let resource = context.resource::<TestResource>().expect("Resource doesn't exist");

        assert_eq!(resource.0, "Hello, World!");
    }

    #[test]
    fn should_mutate_resources() {
        let (_, mut context) = test_init();

        context.add_resource(TestResource("Hello, World!"));
        let mut resource = context.resource_mut::<TestResource>().expect("Resource doesn't exist");
    }
}
