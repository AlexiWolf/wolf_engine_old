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
pub struct WindowSettings {
    /// The desired title for the window.
    pub title: String,

    /// The desired width, in pixels, for the window.
    pub width: usize,

    /// The desired height, in pixels, for the window.
    pub height: usize,

    /// The desired fullscreen mode, if any, for the window.
    ///
    /// Set to `None` for "windowed" mode.
    pub fullscreen_mode: Option<FullscreenMode>,

    /// A flag indicating if the window should be resizable.
    pub is_resizable: bool,
}

impl WindowSettings {
    /// Create a new instance of `WindowSettings` with the default settings.
    ///
    /// Functionally identical to calling [`WindowSettings::default()`].
    pub fn new() -> Self {
        Self::default()
    }
}

/// Provides builder-style methods for configuring the window.
impl WindowSettings {
    /// Set the title of the window.
    pub fn with_title<T: Into<String>>(mut self, title: T) -> Self {
        self.title = title.into();
        self
    }

    /// Set the size, in pixels, of the window.
    pub fn with_size<T: Into<WindowDimensions>>(mut self, size: T) -> Self {
        let dimensions: WindowDimensions = size.into();
        self.width = dimensions.width;
        self.height = dimensions.height;
        self
    }

    /// Set the [`FullscreenMode`], if any, for the window.
    ///
    /// Use [`None`] to set to "windowed" mode.
    pub fn with_fullscreen(self) -> Self {
        self.with_fullscreen_mode(Some(FullscreenMode::Fullscreen))
    }

    /// Set the window to borderless fullscreen mode.
    pub fn with_borderless_fullscreen(self) -> Self {
        self.with_fullscreen_mode(Some(FullscreenMode::Borderless))
    }

    /// Set the window to exclusive fullscreen mode.
    pub fn with_fullscreen_mode(mut self, fullscreen_mode: Option<FullscreenMode>) -> Self {
        self.fullscreen_mode = fullscreen_mode;
        self
    }

    /// Set the window to windowed mode.
    pub fn with_windowed(self) -> Self {
        self.with_fullscreen_mode(None)
    }

    /// Set the resizable flag.
    pub fn with_resizable(mut self, is_resizable: bool) -> Self {
        self.is_resizable = is_resizable;
        self
    }
}

impl Default for WindowSettings {
    fn default() -> Self {
        Self {
            title: "Wolf Engine - Untitled Window".to_string(),
            width: 1280,
            height: 720,
            fullscreen_mode: None,
            is_resizable: true,
        }
    }
}
