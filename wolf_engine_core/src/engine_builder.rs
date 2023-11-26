use std::marker::PhantomData;

use crate::ecs::{ResourcesBuilder, World};
use crate::prelude::*;

/// Represents the [`EventLoop`]-[`Context`] pair that makes up "the engine."
pub type Engine<E> = (EventLoop<E>, Context<E>);

/// Provides a common interface for configuring the [`Engine`].
pub struct EngineBuilder<E: UserEvent> {
    resources: ResourcesBuilder,
    _event_type: PhantomData<E>,
}

impl<E: UserEvent> EngineBuilder<E> {
    pub(crate) fn new() -> Self {
        Self {
            resources: ResourcesBuilder::default(),
            _event_type: PhantomData,
        }
    }

    /// Add resources to the [`Engine`].
    pub fn with_resources(mut self, resources: ResourcesBuilder) -> Self {
        self.resources = resources;
        self
    }

    /// Consume the builder, and return the [`Engine`] created from it.
    pub fn build(mut self) -> Engine<E> {
        let event_loop = EventLoop::new();
        self.resources.add_resource(event_loop.event_sender());
        let context = Context {
            world: World::default(),
            resources: self.resources.build(),
            event_sender: event_loop.event_sender(),
        };
        (event_loop, context)
    }
}
