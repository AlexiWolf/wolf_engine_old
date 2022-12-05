#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

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
    pub fn with_title(mut self, title: &str) -> Self {
        self.title = title.to_string();
        self
    }

    /// Set the size, in pixels, of the window.
    pub fn with_size(mut self, size: (usize, usize)) -> Self {
        self.width = size.0;
        self.height = size.1;
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

#[cfg(test)]
mod window_settings_tests {
    pub use super::*;

    #[test]
    fn should_implement_default_trait() {
        let window_settings = WindowSettings::default();
        assert_eq!(
            window_settings,
            WindowSettings {
                title: "Wolf Engine - Untitled Window".to_string(),
                width: 1280,
                height: 720,
                fullscreen_mode: None,
                is_resizable: true,
            }
        );
    }

    #[test]
    fn should_set_title() {
        let settings = WindowSettings::new().with_title("Test Title");
        assert_eq!(settings.title, "Test Title");
    }

    #[test]
    fn should_set_size() {
        let settings = WindowSettings::new().with_size((800, 600));
        assert_eq!(settings.width, 800);
        assert_eq!(settings.height, 600);
    }

    #[test]
    fn should_set_fullscreen_mode() {
        let settings = WindowSettings::new().with_fullscreen_mode(Some(FullscreenMode::Fullscreen));
        assert_eq!(settings.fullscreen_mode, Some(FullscreenMode::Fullscreen));
    }

    #[test]
    fn should_set_fullscreen() {
        let settings = WindowSettings::new().with_fullscreen();
        assert_eq!(settings.fullscreen_mode, Some(FullscreenMode::Fullscreen));
    }

    #[test]
    fn should_set_to_borderless_fullscreen() {
        let settings = WindowSettings::new().with_borderless_fullscreen();
        assert_eq!(settings.fullscreen_mode, Some(FullscreenMode::Borderless));
    }

    #[test]
    fn should_set_to_window_mode() {
        let settings = WindowSettings::new().with_fullscreen();
        assert_eq!(settings.fullscreen_mode, Some(FullscreenMode::Fullscreen));

        let settings = settings.with_windowed();
        assert_eq!(settings.fullscreen_mode, None);
    }

    #[test]
    fn should_set_to_resizable() {
        let settings = WindowSettings::new().with_resizable(false);
        assert_eq!(settings.is_resizable, false);
    }
}

#[cfg(test)]
#[cfg(feature = "serde")]
mod window_settings_serde_implementation_tests {
    use super::*;

    #[test]
    fn should_implement_deserialize() {
        let toml_str = r#"
            title = "Hello, world"
            width = 1920
            height = 1080
            is_resizable = true
        "#;
        let window_settings: WindowSettings = toml::from_str(toml_str).unwrap();
        assert_eq!(window_settings.title, "Hello, world");
    }

    #[test]
    fn should_implement_serialize() {
        let window_settings = WindowSettings::default();
        let toml_str = toml::to_string(&window_settings).unwrap();

        assert_eq!(
            toml_str,
            "title = \"Wolf Engine - Untitled Window\"\n".to_owned()
                + &"width = 1280\n".to_owned()
                + &"height = 720\n".to_owned()
                + &"is_resizable = true\n".to_owned(),
        );
    }
}
