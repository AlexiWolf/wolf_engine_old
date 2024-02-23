use shared_resources::Resource;

use crate::prelude::*;
use crate::plugins::*;
use crate::resources::Resources;

/// Represents the [`EventLoop`]-[`Context`] pair that makes up "the engine."
pub type Engine = (EventLoop, Context);

/// Provides a common interface for configuring the [`Engine`].
pub struct EngineBuilder {
    resources: Resources,
    plugin_loader: PluginLoader,
}

impl EngineBuilder {
    pub(crate) fn new() -> Self {
        Self {
            resources: Resources::default(),
            plugin_loader: PluginLoader::new(),
        }
    }

    pub fn with_plugin<T: Plugin + 'static>(&mut self, plugin: T) -> &mut Self {
        self.plugin_loader.add_plugin(Box::from(plugin));
        self
    }

    pub fn with_resource<T: Resource>(&mut self, resource: T) -> &mut Self {
        self.resources.insert(resource);
        self
    }

    /// Consume the builder, and return the [`Engine`] created from it.
    pub fn build(&mut self) -> Result<Engine, String> {
        let event_loop = EventLoop::new();
        let mut plugin_loader = std::mem::take(&mut self.plugin_loader);
        plugin_loader.load_plugins(self)?;
        let mut resources = std::mem::take(&mut self.resources);
        resources.insert(event_loop.event_sender().clone());
        let context = Context {
            resources,
            event_sender: event_loop.event_sender().clone(),
        };
        Ok((event_loop, context))
    }
}

#[cfg(test)]
mod engin_builder_tests {
    use crate::{events::MainEventSender, plugins::MockPlugin};

    #[test]
    fn should_add_resources() {
        let (_event_loop, context) = crate::init().with_resource(0).build().unwrap();

        assert!(
            context.resources().get::<i32>().is_ok(),
            "The resources were not used"
        );
    }
    
    #[test]
    fn should_add_plugins() {
        let mut plugin = MockPlugin::new();
        plugin.expect_load().once().return_const(Ok(()));
        let _engine = crate::init()
            .with_plugin(plugin)
            .build()
            .unwrap();
    }

    #[test]
    fn should_add_event_sender_resource_by_default() {
        let (_event_loop, context) = crate::init().build().unwrap();
        let _event_sender = context
            .resources()
            .get_mut::<MainEventSender>()
            .expect("No event sender was added.");
    }
}
