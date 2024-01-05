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
pub struct Context {
    pub(crate) resources: Resources,
    pub(crate) event_sender: Arc<dyn EventSender<Box<dyn Event>>>,
}

impl Context {
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
        self.event_sender.send_event(Box::from(EngineEvent::Quit)).ok();
    }
}

impl HasEventSender<Box<dyn Event>> for Context {
    fn event_sender(&self) -> Arc<dyn EventSender<Box<dyn Event>>> {
        self.event_sender.clone()
    }
}

#[cfg(test)]
mod context_tests {
    #[test]
    fn should_have_accessors() {
        let (_, mut context) = crate::init().build();
        {
            let _resources = context.resources();
        }
        {
            let _mut_resources = context.resources_mut();
        }
    }
}
