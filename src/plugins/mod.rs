//! Provides built-in [Plugin](crate::Plugin) implementations.

mod core_plugin;
mod puffin_plugin;

pub(crate) use core_plugin::*;
pub use puffin_plugin::*;
