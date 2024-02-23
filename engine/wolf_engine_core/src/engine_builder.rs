use std::marker::PhantomData;

use shared_resources::Resource;

use crate::plugins::*;
use crate::prelude::*;
use crate::resources::Resources;

/// Represents the [`EventLoop`]-[`Context`] pair that makes up "the engine."
pub type Engine = (EventLoop, Context);

pub mod state {
    pub struct PreBuild;
    pub struct Building;
}

/// Provides a common interface for configuring the [`Engine`].
pub struct EngineBuilder<State> {
    resources: Resources,
    plugin_loader: PluginLoader,
    _state: PhantomData<State>,
}

impl EngineBuilder<state::PreBuild> {
    pub fn with_plugin<T: Plugin + 'static>(&mut self, plugin: T) -> &mut Self {
        self.plugin_loader.add_plugin(Box::from(plugin));
        self
    }

    /// Consume the builder, and return the [`Engine`] created from it.
    pub fn build(&mut self) -> Result<Engine, String> {
        let (mut engine_builder, mut plugin_loader) = self.start_build();
        let event_loop = EventLoop::new();
        engine_builder.with_resource(event_loop.event_sender().clone());
        plugin_loader.load_plugins(&mut engine_builder)?;
        let context = Context {
            resources: std::mem::take(&mut engine_builder.resources),
            event_sender: event_loop.event_sender().clone(),
        };
        Ok((event_loop, context))
    }

    fn start_build(&mut self) -> (EngineBuilder<state::Building>, PluginLoader) {
        let plugin_loader = std::mem::take(&mut self.plugin_loader);
        let resources = std::mem::take(&mut self.resources);
        let engine_builder = EngineBuilder::<state::Building> {
            resources,
            plugin_loader: PluginLoader::default(),
            _state: PhantomData::default(),
        };
        (engine_builder, plugin_loader)
    }
}

impl<State> EngineBuilder<State> {
    pub(crate) fn new() -> EngineBuilder<state::PreBuild> {
        EngineBuilder::<state::PreBuild> {
            resources: Resources::default(),
            plugin_loader: PluginLoader::new(),
            _state: PhantomData::default(),
        }
    }

    pub fn with_resource<T: Resource>(&mut self, resource: T) -> &mut Self {
        self.resources.insert(resource);
        self
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
        let _engine = crate::init().with_plugin(plugin).build().unwrap();
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
