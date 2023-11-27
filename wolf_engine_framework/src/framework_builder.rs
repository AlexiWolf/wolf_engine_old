use crate::plugins::{Plugin, PluginLoader};

use wolf_engine_core::ecs::systems::Resource;
use wolf_engine_core::ecs::Resources;
use wolf_engine_core::events::UserEvent;
use wolf_engine_core::Engine;

/// Provides a way to configure the [`Engine`] before startup.
///
/// This is similar to the [`EngineBuilder`](wolf_engine_core::EngineBuilder), except it handles
/// all of the setup for you, and provides some additional features (like a plugin system.)
pub struct FrameworkBuilder<E: UserEvent> {
    resources: Resources,
    plugin_loader: PluginLoader<E>,
}

impl<E: UserEvent> FrameworkBuilder<E> {
    pub(crate) fn new() -> Self {
        Self {
            resources: Resources::default(),
            plugin_loader: PluginLoader::new(),
        }
    }

    /// Adds a [`Plugin`] to the engine.
    ///
    /// **Note:** Plugins are loaded when [`FrameworkBuilder::build()`] is called.
    pub fn with_plugin<P: Plugin<E> + 'static>(&mut self, plugin: P) -> &mut Self {
        self.plugin_loader.add_plugin(Box::from(plugin));
        self
    }

    /// Adds a [`Resource`] of type `T` to the engine's [`Resources`].
    ///
    /// **Note:** If a provided type is already in the store, it will be silently overwritten. This
    /// behavior is consistent with [`Resources::insert()`].
    pub fn with_resource<T: Resource>(&mut self, resource: T) -> &mut Self {
        self.resources.insert(resource);
        self
    }

    /// Creates a new instance of [`Engine`] from the builder.
    pub fn build(&mut self) -> Result<Engine<E>, String> {
        let mut plugin_loader = std::mem::replace(&mut self.plugin_loader, PluginLoader::new());
        match plugin_loader.load_plugins(self) {
            Ok(_) => (),
            Err(error) => return Err(error),
        }
        let resources = std::mem::take(&mut self.resources);
        Ok(wolf_engine_core::init().with_resources(resources).build())
    }
}
