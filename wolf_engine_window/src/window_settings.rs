#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Provides the settings used to create a window.
#[derive(Default, Debug, PartialEq, Eq)]
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
