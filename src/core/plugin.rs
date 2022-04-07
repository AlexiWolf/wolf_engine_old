use std::any::{type_name, Any};

use log::*;

#[cfg(test)]
use mockall::automock;

use crate::EngineBuilder;

pub type PluginResult = Result<EngineBuilder, PluginError>;
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
    fn setup(&mut self, engine_builder: EngineBuilder) -> PluginResult;
    fn name(&self) -> &'static str {
        type_name::<Self>()
    }
}

pub type Plugins = Vec<Box<dyn Plugin>>;

pub struct PluginLoader {
    plugins: Plugins,
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
        assert!(plugin_loader.is_empty(), "The plugin loader must start empty");
    }
}
