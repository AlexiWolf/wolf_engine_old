use crate::plugins::Plugin;

use wolf_engine_core::ecs::systems::Resource;
use wolf_engine_core::ecs::ResourcesBuilder;
use wolf_engine_core::events::UserEvent;
use wolf_engine_core::Engine;

pub struct FrameworkBuilder<E: UserEvent> {
    resource_builder: ResourcesBuilder,
    plugins: Vec<Box<dyn Plugin<E>>>,
}

impl<E: UserEvent> FrameworkBuilder<E> {
    pub(crate) fn new() -> Self {
        Self {
            plugins: Vec::new(),
            resource_builder: ResourcesBuilder::default(),
        }
    }

    pub fn with_plugin<P: Plugin<E> + 'static>(mut self, plugin: P) -> Self {
        self.plugins.push(Box::from(plugin));
        self
    }

    pub fn with_resource<T: Resource + 'static>(mut self, resource: T) -> Self {
        self.resource_builder.add_resource(resource);
        self
    }

    pub fn build(mut self) -> Engine<E> {
        let plugins = std::mem::take(&mut self.plugins);

        for mut plugin in plugins {
            self = plugin.load(self).expect("Failed to load plugin");
        }

        wolf_engine_core::init()
            .with_resources(self.resource_builder)
            .build()
    }
}
