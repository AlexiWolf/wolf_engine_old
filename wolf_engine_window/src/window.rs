use crate::*;

use raw_window_handle::{HasRawWindowHandle, HasRawDisplayHandle};

#[cfg(test)]
use mockall::{mock, automock};

#[cfg(test)]
use raw_window_handle::{RawWindowHandle, RawDisplayHandle};

/// Provides a high-level API for creating, and working with [`Windows`](Window).
#[cfg_attr(test, automock(type Window = MockWindow;))]
pub trait WindowBackend {
    
    /// The [`Window`] type used by this window implementation.
    type Window: Window;
    
    /// Create a window with the provided settings.
    ///
    /// Returns a [`Result`] containing a [`Window`], or a message explaining what went wrong.
    fn create_window(&mut self, settings: WindowSettings) -> Result<Self::Window, String>;
}

/// Provides a high-level, back-end agnostic window API. 
///
/// # Examples
///
/// A new window is created by passing [`WindowSettings`] to a [`WindowBackend`].
pub trait Window: HasRawWindowHandle + HasRawDisplayHandle {

    /// Return the window's unique id.
    fn id(&self) -> WindowId;

    /// Return the window's title.
    fn title(&self) -> String;
    
    /// Set the window's title.
    fn set_title<T: Into<String> + 'static>(&mut self, title: T);

    /// Return the window's width, in pixels.
    fn width(&self) -> usize;

    /// Return the window's height, in pixels.
    fn height(&self) -> usize;

    /// Return the window's size.
    fn size(&self) -> WindowDimensions;

    /// Set the window's size.
    fn set_size<T: Into<WindowDimensions> + 'static>(&mut self, size: T);

    /// Return the window's [`FullscreenMode`] if there is one.
    fn fullscreen_mode(&self) -> Option<FullscreenMode>;

    /// Set the window's [`FullscreenMode`].
    ///
    /// Setting this value to `None` for "windowed" mode.
    fn set_fullscreen_mode(&mut self, fullscreen_mode: Option<FullscreenMode>);

    /// Return `true` if the window is in fullscreen mode.
    ///
    /// If the [`FullscreenMode`] is [`Some`], `true` is returned.
    /// If [`None`], then `false` is returned.
    fn is_fullscreen(&self) -> bool;
}

#[cfg(test)]
mock! {
    pub Window {} 

    impl Window for Window {
        fn id(&self) -> WindowId;
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

    unsafe impl HasRawWindowHandle for Window {
        fn raw_window_handle(&self) -> RawWindowHandle;
    }

    unsafe impl HasRawDisplayHandle for Window {
        fn raw_display_handle(&self) -> RawDisplayHandle;
    }
}


#[cfg(test)]
pub mod window_api_tests {
    use super::*;
    
    use raw_window_handle::{WebWindowHandle, WebDisplayHandle};

    #[test]
    fn should_have_id_getter() {
        let (mut window, _backend) = mock_window(WindowSettings::default());
        window.expect_id().once().returning(WindowId::new());

        let _id = window.id();
    }

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

    #[test]
    fn should_implement_raw_window_handle() {
        let (mut window, _backend) = mock_window(WindowSettings::default());
        window.expect_raw_window_handle()
            .once()
            .returning(|| RawWindowHandle::Web(WebWindowHandle::empty()));
        let _handle = window.raw_window_handle();
    }

    #[test]
    fn should_implement_raw_display_handle() {
        let (mut window, _backend) = mock_window(WindowSettings::default());
        window.expect_raw_display_handle()
            .once()
            .returning(|| RawDisplayHandle::Web(WebDisplayHandle::empty()));
        let _handle = window.raw_display_handle();
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
