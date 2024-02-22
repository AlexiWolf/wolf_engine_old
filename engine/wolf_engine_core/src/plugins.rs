//! Provides a plugin system for the engine.

use crate::EngineBuilder;

#[cfg(test)]
use mockall::automock;

/// A result type for the plugin system.
pub type PluginResult = Result<(), String>;

/// A module which adds new functionality to the engine.
#[cfg_attr(test, automock)]
pub trait Plugin {
    /// Returns a people-friendly name for the plugin.
    ///
    /// This is mostly used to identify the plugin in logs.  There aren't any specific requirements
    /// for what the name should be, and it may change, so you probably shouldn't use this to
    /// uniquely identify plugins.
    fn name(&self) -> &str;

    /// Loads the plugin using the provided [`FrameworkBuilder`].
    ///
    /// The plugin does all setup here.
    ///
    /// **Note:** Plugins shouldn't try to load other plugins using the builder.  At this point in
    /// the setup process, it's not possible to add additional plugins.  Nothing will happen if you
    /// try.
    fn load(&mut self, builder: &mut EngineBuilder) -> PluginResult;
}
