//! Provides a high-level, back-end agnostic window API for [Wolf
//! Engine](https://docs.rs/wolf_engine/latest).

mod window_settings;
pub use window_settings::*;

#[cfg(test)]
use mockall::automock;

pub mod prelude {
    pub use super::*;
}

#[cfg_attr(test, automock(type Window = MockWindow;))]
pub trait WindowBackend {
    type Window: Window;

    fn create_window(&mut self, settings: WindowSettings) -> Result<Self::Window, String>;
}

#[cfg_attr(test, automock)]
pub trait Window {
    fn title(&self) -> String;
    fn set_title(&mut self, title: &str);
    fn width(&self) -> usize;
    fn height(&self) -> usize;
    fn size(&self) -> (usize, usize);
    fn set_size(&mut self, size: (usize, usize));
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
        window.expect_set_title().once().return_const(());

        let _title = window.title();
        window.set_title("Hello, World!");
    }

    #[test]
    fn should_have_size_settors_and_getters() {
        let (mut window, _backend) = mock_window(WindowSettings::default());
        window.expect_width().once().returning(|| 800);
        window.expect_height().once().returning(|| 600);
        window.expect_size().once().returning(|| (800, 600));
        window.expect_set_size().once().return_const(());

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
