pub struct WindowSettings {
    pub title: String,
    pub width: usize,
    pub height: usize,
    pub is_fullscreen: bool,
    pub is_borderless: bool,
    pub is_resizable: bool,
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
}
