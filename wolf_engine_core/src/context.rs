use std::sync::Arc;

use crate::ecs::*;
use crate::events::*;

/// Provides a container for Wolf Engine's user-facing data.
///
/// Wolf Engine consists of two main parts: The `Context` (You are here!), and the
/// [`EventLoop`](crate::EventLoop`).  Together, these two parts make up what we refer to as
/// ["the engine"](crate::Engine).
///
/// The Context owns all engine data, including resources, and the game world.
pub struct Context<E: UserEvent> {
    pub(crate) world: World,
    pub(crate) resources: Resources,
    pub(crate) event_sender: Arc<dyn EventSender<Event<E>>>,
}

impl<E: UserEvent> Context<E> {
    /// Returns an immutable reference to the world.
    pub fn world(&self) -> &World {
        &self.world
    }

    /// Returns a mutable reference to the world.
    pub fn world_mut(&mut self) -> &mut World {
        &mut self.world
    }

    /// Returns an immutable reference to engine resources.
    pub fn resources(&self) -> &Resources {
        &self.resources
    }

    /// Returns a mutable reference to engine resources.
    pub fn resources_mut(&mut self) -> &mut Resources {
        &mut self.resources
    }

    /// Sends a [Quit Event](Event::Quit) to trigger an engine shutdown.
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
    #[test]
    fn should_have_accessors() {
        let (_, mut context) = crate::init::<()>().build();
        {
            let _world = context.world();
            let _resources = context.resources();
        }
        {
            let _world_mut = context.world_mut();
            let _mut_resources = context.resources_mut();
        }
    }
}
