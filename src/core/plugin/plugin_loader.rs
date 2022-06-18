use crate::{EngineBuilder, Plugin};
use log::*;

/// A collection of Plugins.
pub type Plugins = Vec<Box<dyn Plugin>>;

/// Provides [Plugin] loading for the [EngineBuilder].
///
/// [Plugin]s are added the the plugin loader, then loaded in the order they were added
/// when [PluginLoader::load_all()] is called.
pub struct PluginLoader {
    plugins: Plugins,
}

impl Default for PluginLoader {
    fn default() -> Self {
        Self::new()
    }
}

impl PluginLoader {
    /// Create an empty plugin loader.
    pub fn new() -> Self {
        Self {
            plugins: Vec::new(),
        }
    }

    /// Add a [Plugin] to the queue.
    pub fn add(&mut self, plugin: Box<dyn Plugin>) {
        self.plugins.push(plugin);
    }

    /// Returns the number of plugins to be loaded.
    pub fn len(&self) -> usize {
        self.plugins.len()
    }

    /// Returns true if there are no plugins added to the plugin loader.
    pub fn is_empty(&self) -> bool {
        self.plugins.is_empty()
    }

    /// Consume the Plugin Loader and load all plugins in the order they were added.
    ///
    /// Information about which plugins are being loaded, as well as their status is
    /// logged as [debug information](debug).
    pub fn load_all(mut self, mut engine_builder: EngineBuilder) -> Result<EngineBuilder, String>{
        for plugin in self.plugins.iter_mut() {
            debug!("Now loading plugin: {}", plugin.name());
            engine_builder = match plugin.setup(engine_builder) {
                Ok(engine_builder) => {
                    debug!("Successfully loaded plugin: {}", plugin.name());
                    engine_builder
                }
                Err((error_message, _)) => {
                    let error = format!("Failed to load {}: {}", plugin.name(), error_message);
                    error!("{}", error);
                    return Err(error) 
                }
            }
        }
        Ok(engine_builder)
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

        let _engine_builder = plugin_loader.load_all(EngineBuilder::new()).unwrap();
    }

    fn mock_plugin() -> MockPlugin {
        let mut plugin = MockPlugin::new();
        plugin.expect_setup().once().returning(Ok);
        plugin
    }

    #[test]
    fn should_return_error_on_plugin_failure() {
        let mut plugin_loader = PluginLoader::new();
        let mut plugin = MockPlugin::new(); 
        plugin.expect_setup()
            .once()
            .returning(|engine_builder| Err(("Test error", engine_builder)));
        plugin_loader.add(Box::from(plugin));

        let loader_result = plugin_loader.load_all(EngineBuilder::new());
    
        assert!(loader_result.is_err());
        assert_eq!(loader_result.unwrap_err(), "Failed to load Test Plugin: Test Error");
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
