mod plugin_loader;

pub use plugin_loader::*;

use crate::EngineBuilder;
#[cfg(test)]
use mockall::automock;
use std::any::{type_name, Any};

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
