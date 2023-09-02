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
pub trait Window {}
