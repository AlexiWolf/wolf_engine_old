use crate::plugins::{Plugin, PluginLoder};

use wolf_engine_core::ecs::systems::Resource;
use wolf_engine_core::ecs::Resources;
use wolf_engine_core::events::UserEvent;
use wolf_engine_core::Engine;

pub struct FrameworkBuilder<E: UserEvent> {
    resources: Resources,
    plugin_loader: PluginLoder<E>,
}

impl<E: UserEvent> FrameworkBuilder<E> {
    pub(crate) fn new() -> Self {
        Self {
            resources: Resources::default(),
            plugin_loader: PluginLoder::new(),
        }
    }

    pub fn with_plugin<P: Plugin<E> + 'static>(&mut self, plugin: P) -> &mut Self {
        self.plugin_loader.add_plugin(Box::from(plugin));
        self
    }

    pub fn with_resource<T: Resource>(&mut self, resource: T) -> &mut Self {
        self.resources.insert(resource);
        self
    }

    pub fn build(&mut self) -> Result<Engine<E>, String> {
        let mut plugin_loader = std::mem::replace(&mut self.plugin_loader, PluginLoder::new());
        match plugin_loader.load_plugins(self) {
            Ok(_) => (),
            Err(error) => return Err(error),
        }
        let resource_builder = std::mem::take(&mut self.resources);
        Ok(wolf_engine_core::init()
            .with_resources(resource_builder)
            .build())
    }
}
