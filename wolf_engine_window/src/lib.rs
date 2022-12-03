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
    }
}
