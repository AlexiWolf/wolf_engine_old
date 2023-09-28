use std::sync::Arc;

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
    schedule: Schedule,
    event_sender: Arc<dyn EventSender<Event<E>>>,
}

impl<E: UserEvent> Context<E> {
    /// Create a new `Context` from the provided [`EventQueue`] and data.
    pub(crate) fn new(event_queue: &dyn EventQueue<Event<E>>, resources: Resources, schedule: Schedule, world: World) -> Self {
        Self {
            world,
            resources,
            schedule,
            event_sender: event_queue.event_sender(),
        }
    }

    pub fn world(&self) -> &World {
        &self.world
    }

    pub fn world_mut(&mut self) -> &mut World {
        &mut self.world
    }

    pub fn resources(&self) -> &Resources {
        &self.resources 
    }

    pub fn resources_mut(&mut self) -> &mut Resources {
        &mut self.resources
    }

    pub fn schedule(&self) -> &Schedule {
        &self.schedule
    }

    pub fn schedule_mut(&mut self) -> &mut Schedule {
        &mut self.schedule 
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
    use super::*;

    use crate::EventLoop;

    pub fn init() -> (EventLoop<()>, Context<()>) {
        let event_loop = EventLoop::<()>::new();
        let context = Context::new(&event_loop, Resources::default(), Schedule::builder().build());
        (event_loop, context)
    }

    #[test]
    fn should_have_world_accessors() {
        let (_, mut context) = init();
        { 
            let _world = context.world(); 
        }
        {
            let _world_mut = context.world_mut();
        }
    }

    #[test]
    fn should_have_resources_accessors() {
        let (_, mut context) = init();
        { 
            let _resources = context.resources(); 
        }
        {
            let _mut_resources = context.resources_mut();
        }
    }

    #[test]
    fn should_have_schedule_accessors() {
        let (_, mut context) = init();
        { 
            let _schedule = context.schedule(); 
        }
        {
            let _schedule = context.schedule_mut();
        }
    }
}
