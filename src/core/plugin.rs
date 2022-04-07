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
pub trait Plugin {
    fn setup(&mut self, engine_builder: EngineBuilder) -> PluginResult;
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

    pub fn load_all(mut self, mut engine_builder: EngineBuilder) -> EngineBuilder {
        for plugin in self.plugins.iter_mut() {
            engine_builder = plugin.setup(engine_builder).ok().unwrap();
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

        assert_eq!(plugin_loader.len(), 1, "The plugin was not added to the PluginLoader");
    }

    #[test]
    fn should_load_plugins_on_load_all_call() { 
        let mut plugin_a = MockPlugin::new(); 
        let mut plugin_b = MockPlugin::new();
        expect_setup(&mut plugin_a);
        expect_setup(&mut plugin_b);
        let mut plugin_loader = PluginLoader::new();
        plugin_loader.add(Box::from(plugin_a));
        plugin_loader.add(Box::from(plugin_b));
       
        let _engine_builder = plugin_loader.load_all(EngineBuilder::new());
    }

    fn expect_setup(plugin: &mut MockPlugin) {
        plugin.expect_setup()
            .once()
            .returning(|engine_builder| Ok(engine_builder));
    }
}

