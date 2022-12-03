#[derive(Debug, PartialEq, Eq)]
pub struct WindowSettings {
    pub title: String,
    pub width: usize,
    pub height: usize,
    pub is_fullscreen: bool,
    pub is_borderless: bool,
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
}

impl Default for WindowSettings {
    fn default() -> Self {
        Self {
            title: "Wolf Engine - Untitled Window".to_string(),
            width: 1280,
            height: 720,
            is_fullscreen: false,
            is_borderless: false,
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
                is_fullscreen: false,
                is_borderless: false,
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
}
