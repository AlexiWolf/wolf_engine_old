use std::sync::Arc;

use crate::events::*;

/// Provides a container for Wolf Engine's user-facing data.
///
/// Under the hood, Wolf Engine consists of two main parts: The `Context` (You are here!), and the
/// [`EventLoop`](crate::EventLoop`).  Together, these two parts make up what we refer to as
/// "the engine."
///
/// The Context owns all engine data, sub-systems, and the link to the Event-Loop through which
/// all events are sent.  As far as the end-user is concerned, the Context *is* the engine.
///
/// # Examples
///
/// ## Creating a `Context`
///
/// A good chunk of Wolf Engine's API depends on the Context.  As such, you will very likely need
/// to create one *before* trying to use any of the rest of the API.  You can initialize a Context,
/// along with its associated [`EventLoop`](crate::EventLoop), by calling [`wolf_engine::init()`](crate::init()).
///
/// ```
/// # use wolf_engine_core as wolf_engine;
/// let (mut event_loop, mut context) = wolf_engine::init::<()>(());
/// ```
///
/// ## Context Data
///
/// It's possible to extend the engine with custom functionality by using Context Data.  
///
/// Context data can be any type, and is provided to [`wolf_engine::init()`](crate::init()) at
/// startup.  This data is mostly intended to be *engine data* such as sub-systems, however,
/// there are no specific requirements around what the data is used for.  
///
/// If most cases, context data is only useful to those writing extensions for Wolf Engine, or for
/// those who are building their own framework on top of Wolf Engine's Core API.  Users of
/// the `framework` feature will use the Context data provided by the framework.
///
/// ```
/// # use wolf_engine_core as wolf_engine;
/// #
/// // Create a type for your custom data.
/// struct CustomContextData {
///     // Custom sub-system data goes here.
/// }
/// #
/// # impl CustomContextData {
/// #   pub fn new() -> Self { Self {} }
/// # }
///
/// // Initialize the engine with your custom data.
/// let (mut event_loop, mut context) = wolf_engine::init::<()>(CustomContextData::new());
/// ```
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
        let event_loop = EventLoop::<()>::new();
        let context = Context::new(&event_loop);
    }
}
