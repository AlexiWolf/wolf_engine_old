//! Provides a high-level, back-end agnostic window API for [Wolf
//! Engine](https://docs.rs/wolf_engine/latest).

mod window_settings;
pub use window_settings::*;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(test)]
use mockall::automock;

pub mod prelude {
    pub use super::*;
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct WindowDimensions {
    pub width: usize,
    pub height: usize,
}

impl WindowDimensions {
    pub fn new(width: usize, height: usize) -> Self {
        Self { width, height }
    }
}

impl From<(usize, usize)> for WindowDimensions {
    fn from(dimensions: (usize, usize)) -> Self {
        Self::new(dimensions.0.into(), dimensions.1.into())
    }
}

#[cfg(test)]
mod window_dimensions_tests {
    use super::*;

    #[test]
    pub fn should_have_width_and_height() {
        let dimensions = WindowDimensions::new(800, 600);
        assert_eq!(dimensions.width, 800);
        assert_eq!(dimensions.height, 600);
    }

    #[test]
    fn should_convert_from_tuple() {
        let dimensions: WindowDimensions = (800, 600).into();
        assert_eq!(dimensions.width, 800);
        assert_eq!(dimensions.height, 600);
    }

    #[cfg(feature = "serde")]
    #[test]
    fn should_implement_deserialize() {
        let toml_str = r#"
        width = 800
        height = 600
        "#;
        let dimensions: WindowDimensions = toml::from_str(toml_str).unwrap();
        assert_eq!(dimensions.width, 800);
        assert_eq!(dimensions.height, 600);
    }

    #[cfg(feature = "serde")]
    #[test]
    fn should_implement_serialize() {
        let dimensions = WindowDimensions::new(1280, 720);
        let toml_str = toml::to_string(&dimensions).unwrap();

        assert_eq!(
            toml_str,
            "width = 1280\n".to_owned() + &"height = 720\n".to_owned()
        );
    }
}

#[cfg_attr(test, automock(type Window = MockWindow;))]
pub trait WindowBackend {
    type Window: Window;

    fn create_window(&mut self, settings: WindowSettings) -> Result<Self::Window, String>;
}

#[cfg_attr(test, automock)]
pub trait Window {
    fn title(&self) -> String;
    fn set_title<T: Into<String> + 'static>(&mut self, title: T);
    fn width(&self) -> usize;
    fn height(&self) -> usize;
    fn size(&self) -> WindowDimensions;
    fn set_size<T: Into<WindowDimensions> + 'static>(&mut self, size: T);
    fn fullscreen_mode(&self) -> Option<FullscreenMode>;
    fn set_fullscreen_mode(&mut self, fullscreen_mode: Option<FullscreenMode>);
    fn is_fullscreen(&self) -> bool;
}

#[cfg(test)]
pub mod window_api_tests {
    use super::*;

    #[test]
    fn should_have_title_setter_and_accessor() {
        let (mut window, _backend) = mock_window(WindowSettings::default());
        window
            .expect_title()
            .once()
            .returning(|| "Test".to_string());
        window.expect_set_title::<&str>().once().return_const(());

        let _title = window.title();
        window.set_title("Hello, World!");
    }

    #[test]
    fn should_have_size_settors_and_getters() {
        let (mut window, _backend) = mock_window(WindowSettings::default());
        window.expect_width().once().returning(|| 800);
        window.expect_height().once().returning(|| 600);
        window
            .expect_size()
            .once()
            .returning(|| WindowDimensions::new(800, 600));
        window
            .expect_set_size::<(usize, usize)>()
            .once()
            .return_const(());

        let _width = window.width();
        let _height = window.height();
        let _size = window.size();
        window.set_size((800, 600));
    }

    #[test]
    fn should_have_fullscreen_mode_getters_and_setters() {
        let (mut window, _backend) = mock_window(WindowSettings::default());
        window.expect_fullscreen_mode().once().returning(|| None);
        window.expect_set_fullscreen_mode().once().return_const(());
        window.expect_is_fullscreen().once().returning(|| false);

        let _fullscreen_mode = window.fullscreen_mode();
        let _is_fullscreen = window.is_fullscreen();
        window.set_fullscreen_mode(Some(FullscreenMode::Fullscreen));
    }

    fn mock_window(settings: WindowSettings) -> (MockWindow, MockWindowBackend) {
        let mut backend = MockWindowBackend::new();
        backend
            .expect_create_window()
            .once()
            .returning(|_| Ok(MockWindow::new()));
        let window = backend.create_window(settings).unwrap();
        (window, backend)
    }
}
