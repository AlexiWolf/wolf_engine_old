#[derive(Debug, PartialEq, Eq)]
pub enum VideoMode {
    Fullscreen,
}

#[derive(Debug, PartialEq, Eq)]
pub struct WindowSettings {
    pub title: String,
    pub width: usize,
    pub height: usize,
    pub video_mode: Option<VideoMode>,
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

    pub fn with_fullscreen(mut self) -> Self {
        self.is_fullscreen = true;
        self
    }
}

impl Default for WindowSettings {
    fn default() -> Self {
        Self {
            title: "Wolf Engine - Untitled Window".to_string(),
            width: 1280,
            height: 720,
            video_mode: Option<VideoMode>,
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
                video_mode: None,
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
    fn should_set_fullscreen() {
        let settings = WindowSettings::new()
            .with_fullscreen();
        assert_eq!(settings.video_mode, VideoMode::Fullscreen);
    }
}
