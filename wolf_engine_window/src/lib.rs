#[derive(Debug, PartialEq, Eq)]
pub enum FullscreenMode {
    Fullscreen,
    Borderless,
}

#[derive(Debug, PartialEq, Eq)]
pub struct WindowSettings {
    pub title: String,
    pub width: usize,
    pub height: usize,
    pub fullscreen_mode: Option<FullscreenMode>,
    pub is_resizable: bool,
}

impl WindowSettings {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_title(mut self, title: &str) -> Self {
        self.title = title.to_string();
        self
    }

    pub fn with_size(mut self, size: (usize, usize)) -> Self {
        self.width = size.0;
        self.height = size.1;
        self
    }

    pub fn with_fullscreen(self) -> Self {
        self.with_fullscreen_mode(Some(FullscreenMode::Fullscreen))
    }

    pub fn with_borderless_fullscreen(self) -> Self {
        self.with_fullscreen_mode(Some(FullscreenMode::Borderless))
    }

    pub fn with_fullscreen_mode(mut self, fullscreen_mode: Option<FullscreenMode>) -> Self {
        self.fullscreen_mode = fullscreen_mode;
        self
    }

    pub fn with_windowed(mut self) -> Self {
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
        let settings = WindowSettings::new()
            .with_title("Test Title");
        assert_eq!(settings.title, "Test Title");
    }

    #[test]
    fn should_set_size() {
        let settings = WindowSettings::new()
            .with_size((800, 600));
        assert_eq!(settings.width, 800);
        assert_eq!(settings.height, 600);
    }

    #[test]
    fn should_set_fullscreen_mode() {
        let settings = WindowSettings::new()
            .with_fullscreen_mode(Some(FullscreenMode::Fullscreen));
        assert_eq!(settings.fullscreen_mode, Some(FullscreenMode::Fullscreen));
    }

    #[test]
    fn should_set_fullscreen() {
        let settings = WindowSettings::new()
            .with_fullscreen();
        assert_eq!(settings.fullscreen_mode, Some(FullscreenMode::Fullscreen));
    }

    #[test]
    fn should_set_to_borderless_fullscreen() {
        let settings = WindowSettings::new()
            .with_borderless_fullscreen();
        assert_eq!(settings.fullscreen_mode, Some(FullscreenMode::Borderless));
    }

    #[test]
    fn should_set_to_window_mode() {
        let settings = WindowSettings::new()
            .with_windowed();
        assert_eq!(settings.fullscreen_mode, None);
    }
}
