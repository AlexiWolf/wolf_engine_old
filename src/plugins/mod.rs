//! Provides built-in [Plugin](crate::Plugin) implementations.

mod core_plugin;

#[cfg(feature = "profiling")]
mod puffin_plugin;

pub(crate) use core_plugin::*;

#[cfg(feature = "profiling")]
pub use puffin_plugin::*;
