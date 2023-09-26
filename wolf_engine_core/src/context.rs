use std::sync::Arc;

use atomic_refcell::AtomicRef;
use atomic_refcell::AtomicRefMut;

use crate::ecs::*;
use crate::events::*;

/// Provides a container for Wolf Engine's user-facing data.
///
/// Under the hood, Wolf Engine consists of two main parts: The `Context` (You are here!), and the
/// [`EventLoop`](crate::EventLoop`).  Together, these two parts make up what we refer to as
/// "the engine."
///
/// The Context owns all engine data, sub-systems, and the link to the Event-Loop through which
/// all events are sent.  As far as the end-user is concerned, the Context *is* the engine.
pub struct Context<E: UserEvent> {
    world: World,
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

    pub fn world(&self) -> &World {

    }

    pub fn world_mut(&self) -> &mut World {

    }

    pub fn resources(&self) -> &Resources {
        &self.resources 
    }

    pub fn resources_mut(&mut self) -> &mut Resources {
        &mut self.resources
    }

    pub fn insert_resource<T: 'static>(&mut self, resource: T) {
        self.resources.insert(resource);
    }

    pub fn remove_resource<T: 'static>(&mut self) -> Option<T> {
        self.resources.remove::<T>()
    }

    pub fn resource<T: 'static>(&self) -> Option<AtomicRef<T>> {
        self.resources.get::<T>()
    }

    pub fn resource_mut<T: 'static>(&self) -> Option<AtomicRefMut<T>> {
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
mod test_fixtures {
    use super::*;

    use crate::EventLoop;

    pub struct TestResource(pub &'static str);

    pub fn init() -> (EventLoop<()>, Context<()>) {
        let event_loop = EventLoop::<()>::new();
        let context = Context::new(&event_loop);
        (event_loop, context)
    }
}

#[cfg(test)]
mod context_world_tests {
    use super::*;
   
    #[test]
    fn should_have_world_accessors() {
        let (_, mut context) = test_fixtures::init();
        { 
            let _world = context.world(); 
        }
        {
            let _world = context.world_mut();
        }
    }
}

#[cfg(test)]
mod context_resource_tests {
    use super::*;
    use test_fixtures::TestResource;

    #[test]
    fn should_have_resources_accessors() {
        let (_, mut context) = test_fixtures::init();
        { 
            let _resources = context.resources(); 
        }
        {
            let _mut_resources = context.resources_mut();
        }
    }

    #[test]
    fn should_add_resource() {
        let (_, mut context) = test_fixtures::init();

        context.insert_resource(TestResource("Hello, World!"));
        let resource = context
            .resource::<TestResource>()
            .expect("Resource doesn't exist");

        assert_eq!(resource.0, "Hello, World!");
    }

    #[test]
    fn should_mutate_resource() {
        let (_, mut context) = test_fixtures::init();
        context.insert_resource(TestResource("Hello, World!"));

        {
            let mut resource = context
                .resource_mut::<TestResource>()
                .expect("Resource doesn't exist");
            resource.0 = "Hello, changed World!";
        }

        let resource = context
            .resource::<TestResource>()
            .expect("Resource doesn't exist");
        assert_eq!(resource.0, "Hello, changed World!");
    }

    #[test]
    fn should_remove_resource() {
        let (_, mut context) = test_fixtures::init();

        context.insert_resource(TestResource("Hello, World!"));
        assert!(context.resource::<TestResource>().is_some());

        let resource = context.remove_resource::<TestResource>();
        assert!(resource.is_some());
        assert!(context.resource::<TestResource>().is_none());
    }
}
