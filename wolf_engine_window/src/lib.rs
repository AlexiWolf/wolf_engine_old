pub struct WindowSettings {}

impl Default for WindowSettings {
    fn default() -> Self {
        Self {

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
                title: "Wolf Engine - Untitled Window",
                width: 1280,
                height: 720,
                is_fullscreen: false,
                is_borderless: false,
                is_resizable: true,
            }
        );
    }
}
