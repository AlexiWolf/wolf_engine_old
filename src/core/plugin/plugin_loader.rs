use crate::{EngineBuilder, Plugin};
use log::*;

pub type Plugins = Vec<Box<dyn Plugin>>;

pub struct PluginLoader {
    plugins: Plugins,
}

impl Default for PluginLoader {
    fn default() -> Self {
        Self::new()
    }
}

impl PluginLoader {
    pub fn new() -> Self {
        Self {
            plugins: Vec::new(),
        }
    }

    pub fn add(&mut self, plugin: Box<dyn Plugin>) {
        self.plugins.push(plugin);
    }

    pub fn len(&self) -> usize {
        self.plugins.len()
    }

    pub fn is_empty(&self) -> bool {
        self.plugins.is_empty()
    }

    pub fn load_all(mut self, mut engine_builder: EngineBuilder) -> EngineBuilder {
        for plugin in self.plugins.iter_mut() {
            debug!("Now loading plugin: {}", plugin.name());
            engine_builder = match plugin.setup(engine_builder) {
                Ok(engine_builder) => {
                    debug!("Successfully loaded plugin: {}", plugin.name());
                    engine_builder
                }
                Err((error_message, engine_builder)) => {
                    error!(
                        "Failed to load plugin: {}: {}",
                        plugin.name(),
                        error_message
                    );
                    engine_builder
                }
            }
        }
        engine_builder
    }
}

#[cfg(test)]
mod plugin_loader_tests {
    use super::*;
    use crate::MockPlugin;

    #[test]
    fn should_store_added_plugins() {
        let plugin = MockPlugin::new();
        let mut plugin_loader = PluginLoader::new();

        plugin_loader.add(Box::from(plugin));

        assert_eq!(
            plugin_loader.len(),
            1,
            "The plugin was not added to the PluginLoader"
        );
    }

    #[test]
    fn should_load_plugins_on_load_all_call() {
        let mut plugin_loader = PluginLoader::new();
        plugin_loader.add(Box::from(mock_plugin()));
        plugin_loader.add(Box::from(mock_plugin()));

        let _engine_builder = plugin_loader.load_all(EngineBuilder::new());
    }

    fn mock_plugin() -> MockPlugin {
        let mut plugin = MockPlugin::new();
        plugin
            .expect_setup()
            .once()
            .returning(|engine_builder| Ok(engine_builder));
        plugin
    }

    #[test]
    fn should_create_empty_plugin_loader() {
        let plugin_loader = PluginLoader::new();
        assert!(
            plugin_loader.is_empty(),
            "The plugin loader must start empty"
        );
    }
}
