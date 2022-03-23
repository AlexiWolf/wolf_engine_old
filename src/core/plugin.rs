#[cfg(test)]
use mockall::automock;

use crate::EngineBuilder;

/// A plugin for the [EngineBuilder].
///
/// Plugins are loaded at startup and are given ownership over the [EngineBuilder].  
/// From there, they are free to set things up however they need to.  
///
/// Plugins can: 
///
/// - Add `Subcontext`s to the main `Context`.
/// - Remove `Subcontext`s from the main `Context`.
/// - Add other `Plugins`.
/// - Change the engine's `Scheduler`.
/// - Change the engine's `CoreFunction` (
/// - And anything else.
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
