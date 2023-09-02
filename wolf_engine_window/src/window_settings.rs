#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::WindowDimensions;

/// Represents the fullscreen mode.
#[derive(Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum FullscreenMode {
    /// Exclusive fullscreen mode.
    Fullscreen,
    /// Borderless fullscreen mode.
    Borderless,
}

/// Provides the settings used to create a window.
#[derive(Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct WindowSettings {}

impl WindowSettings {
    /// Create a new instance of `WindowSettings` with the default settings.
    ///
    /// Functionally identical to calling [`WindowSettings::default()`].
    pub fn new() -> Self {
        Self::default()
    }
}

impl WindowSettings {}

impl Default for WindowSettings {
    fn default() -> Self {
        Self {}
    }
}
