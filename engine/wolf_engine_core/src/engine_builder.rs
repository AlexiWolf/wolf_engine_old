use shared_resources::Resource;

use crate::plugins::Plugin;
use crate::prelude::*;
use crate::resources::Resources;

/// Represents the [`EventLoop`]-[`Context`] pair that makes up "the engine."
pub type Engine = (EventLoop, Context);

/// Provides a common interface for configuring the [`Engine`].
pub struct EngineBuilder {
    resources: Resources,
}

impl EngineBuilder {
    pub(crate) fn new() -> Self {
        Self {
            resources: Resources::default(),
        }
    }

    pub fn with_plugin<T: Plugin>(&mut self, plugin: T) -> &mut Self {
        self
    }

    pub fn with_resource<T: Resource>(&mut self, resource: T) -> &mut Self {
        self.resources.insert(resource);
        self
    }

    /// Add resources to the [`Engine`].
    pub fn with_resources(&mut self, resources: Resources) -> &mut Self {
        self.resources = resources;
        self
    }

    /// Consume the builder, and return the [`Engine`] created from it.
    pub fn build(&mut self) -> Result<Engine, String> {
        let event_loop = EventLoop::new();
        let mut resources = std::mem::take(&mut self.resources);
        resources.insert(event_loop.event_sender().clone());
        let context = Context {
            resources,
            event_sender: event_loop.event_sender().clone(),
        };
        Ok((event_loop, context))
    }
}
