use std::any::{type_name, Any};

use crate::EngineBuilder;

use log::*;

#[cfg(test)]
use mockall::automock;

/// Indicates if the a [Plugin] has loaded successfully.
pub type PluginResult = Result<EngineBuilder, PluginError>;

/// Indicates the reason a [Plugin] has failed to load.
pub type PluginError = (&'static str, EngineBuilder);

/// Provides additional functionality to the engine.
///
/// Plugins make it easy to extend the engine with new functionality.  Plugins are loaded
/// at startup by the [EngineBuilder].  Plugins are given direct access to the
/// [EngineBuilder] during startup, and can add to it, or configure it in any way they
/// need.
///
/// Among other things, most plugins will:
///
/// - Add `Subcontext`s to the main `Context`.
/// - Remove `Subcontext`s from the main `Context`.
/// - Add other `Plugins`.
/// - Change the engine's `Scheduler`.
/// - Change the engine's `CoreFunction`.
///
/// # Examples
///
/// Plugins are loaded with [EngineBuilder::with_plugin()].
///
/// ```
/// # use wolf_engine::*;
/// #
/// # pub struct MyPlugin;
/// #
/// # impl Plugin for MyPlugin {
/// #     fn setup(&mut self, engine_builder: EngineBuilder) -> PluginResult {
/// #         Ok(engine_builder)
/// #     }
/// # }
/// #
/// EngineBuilder::new()
///     .with_plugin(Box::from(MyPlugin));
/// ```
///
/// ## Creating a Custom Plugin
///
/// You can create a custom plugin by implementing this trait.
///
/// ```
/// # use wolf_engine::*;
/// #
/// pub struct MyPlugin;
///
/// impl Plugin for MyPlugin {
///     fn setup(&mut self, engine_builder: EngineBuilder) -> PluginResult {
///         // Setup logic here.
///         Ok(engine_builder)
///     }
/// }
/// ```
///
/// Ownership over the [EngineBuilder] must be returned back to the caller.
#[cfg_attr(test, automock)]
pub trait Plugin: Any {
    /// Uses the [EngineBuilder] to configure and extend the [Engine](crate::Engine).
    fn setup(&mut self, engine_builder: EngineBuilder) -> PluginResult;

    /// Get the name of the plugin.
    ///
    /// By default the [type name](type_name) for the plugin is used, but there are no
    /// specific requirements for what must be returned.  The plugin name may not be
    /// unique and should not be used to uniquely identify a plugin.
    fn name(&self) -> &'static str {
        type_name::<Self>()
    }
}

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
    pub fn load_all(mut self, mut engine_builder: EngineBuilder) -> Result<EngineBuilder, String> {
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
                    return Err(error);
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
        plugin
            .expect_setup()
            .once()
            .returning(|engine_builder| Err(("Test Error", engine_builder)));
        plugin.expect_name().once().returning(|| "Test Plugin");
        plugin_loader.add(Box::from(plugin));

        let loader_result = plugin_loader.load_all(EngineBuilder::new());

        assert!(loader_result.is_err());
        assert_eq!(
            loader_result.err().unwrap(),
            "Failed to load Test Plugin: Test Error"
        );
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
