use crate::plugins::{Plugin, PluginLoder};

use wolf_engine_core::ecs::systems::Resource;
use wolf_engine_core::ecs::ResourcesBuilder;
use wolf_engine_core::events::UserEvent;
use wolf_engine_core::Engine;

pub struct FrameworkBuilder<E: UserEvent> {
    resource_builder: ResourcesBuilder,
    plugin_loader: PluginLoder<E>,
}

impl<E: UserEvent> FrameworkBuilder<E> {
    pub(crate) fn new() -> Self {
        Self {
            resource_builder: ResourcesBuilder::default(),
            plugin_loader: PluginLoder::new(), 
        }
    }

    pub fn with_plugin<P: Plugin<E> + 'static>(mut self, plugin: P) -> Self {
        self.plugin_loader.add_plugin(Box::from(plugin));
        self
    }

    pub fn with_resource<T: Resource + 'static>(mut self, resource: T) -> Self {
        self.resource_builder.add_resource(resource);
        self
    }

    pub fn build(mut self) -> Engine<E> {
        let plugin_loader = std::mem::take(&mut self.plugin_loader);

        self = plugin_loader.load_plugins(self);

        wolf_engine_core::init()
            .with_resources(self.resource_builder)
            .build()
    }
}
