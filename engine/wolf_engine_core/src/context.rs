use crate::events::mpsc::*;
use crate::events::*;
use crate::resources::Resources;

/// Provides a container for Wolf Engine's user-facing data.
///
/// Wolf Engine consists of two main parts: The `Context` (You are here!), and the
/// [`EventLoop`](crate::EventLoop`).  The context owns the main [`EventSender`], as well as all
/// [`Resources`] for the engine.
pub struct Context {
    pub(crate) resources: Resources,
    pub(crate) event_sender: MpscEventSender<EventBox>,
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

    pub fn event_sender(&self) -> &MpscEventSender<EventBox> {
        &self.event_sender
    }

    /// Sends a [Quit Event](EngineEvent::Quit) to trigger an engine shutdown.
    pub fn quit(&self) {
        self.event_sender
            .send_event(Box::from(EngineEvent::Quit))
            .ok();
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
