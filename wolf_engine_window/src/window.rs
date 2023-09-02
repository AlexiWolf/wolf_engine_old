use crate::*;

use raw_window_handle::{HasRawDisplayHandle, HasRawWindowHandle};

#[cfg(test)]
use mockall::{automock, mock};

#[cfg(test)]
use raw_window_handle::{RawDisplayHandle, RawWindowHandle};

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
