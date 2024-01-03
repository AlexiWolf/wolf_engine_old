use std::marker::PhantomData;
use std::sync::Arc;

use crate::events::*;
use crate::resources::Resources;

/// Provides a container for Wolf Engine's user-facing data.
///
/// Wolf Engine consists of two main parts: The `Context` (You are here!), and the
/// [`EventLoop`](crate::EventLoop`).  Together, these two parts make up what we refer to as
/// ["the engine"](crate::Engine).
///
/// The Context owns all engine data, including resources, and the game world.
pub struct Context<E: UserEvent> {
    pub(crate) resources: Resources,
    pub(crate) event_sender: Arc<dyn EventSender<Box<dyn EventTrait>>>,
    pub(crate) _user_event: PhantomData<E>,
}

impl<E: UserEvent> Context<E> {
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
        self.event_sender.send_event(Box::from(Event::Quit)).ok();
    }
}

impl<E: UserEvent> HasEventSender<Box<dyn EventTrait>> for Context<E> {
    fn event_sender(&self) -> Arc<dyn EventSender<Box<dyn EventTrait>>> {
        self.event_sender.clone()
    }
}

#[cfg(test)]
mod context_tests {
    #[test]
    fn should_have_accessors() {
        let (_, mut context) = crate::init::<()>().build();
        {
            let _resources = context.resources();
        }
        {
            let _mut_resources = context.resources_mut();
        }
    }
}