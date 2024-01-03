use std::marker::PhantomData;

use crate::prelude::*;
use crate::resources::Resources;

/// Represents the [`EventLoop`]-[`Context`] pair that makes up "the engine."
pub type Engine<E> = (EventLoop, Context<E>);

/// Provides a common interface for configuring the [`Engine`].
pub struct EngineBuilder<E: UserEvent> {
    resources: Resources,
    _event_type: PhantomData<E>,
}

impl<E: UserEvent> EngineBuilder<E> {
    pub(crate) fn new() -> Self {
        Self {
            resources: Resources::default(),
            _event_type: PhantomData,
        }
    }

    /// Add resources to the [`Engine`].
    pub fn with_resources(mut self, resources: Resources) -> Self {
        self.resources = resources;
        self
    }

    /// Consume the builder, and return the [`Engine`] created from it.
    pub fn build(mut self) -> Engine<E> {
        let event_loop = EventLoop::new();
        self.resources.insert(event_loop.event_sender());
        let context = Context {
            resources: self.resources,
            event_sender: event_loop.event_sender(),
            _user_event: Default::default(),
        };
        (event_loop, context)
    }
}
