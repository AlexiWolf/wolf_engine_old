#[cfg(test)]
use mockall::automock;

use crate::EngineBuilder;

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
/// #     fn setup(&mut self, engine_builder: EngineBuilder) -> EngineBuilder {
/// #         engine_builder
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
///     fn setup(&mut self, engine_builder: EngineBuilder) -> EngineBuilder {
///         // Setup logic here.
///         engine_builder
///     }
/// }
/// ```
///
/// Ownership over the [EngineBuilder] must be returned back to the caller.
#[cfg_attr(test, automock)]
pub trait Plugin {
    fn setup(&mut self, engine_builder: EngineBuilder) -> EngineBuilder;
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

    pub fn load_all(self, engine_builder: EngineBuilder) -> EngineBuilder {
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

        assert_eq!(plugin_loader.len(), 1, "The plugin was not added to the PluginLoader");
    }

    #[test]
    fn should_load_plugins_on_load_all_call() { 
        let mut plugin = MockPlugin::new(); 
        plugin.expect_setup()
            .once()
            .returning(|engine_builder| engine_builder);
        let mut plugin_loader = PluginLoader::new();
        plugin_loader.add(Box::from(plugin));
        let mut engine_builder = EngineBuilder::new();
     
        engine_builder = plugin_loader.load_all(engine_builder);
    }
}

