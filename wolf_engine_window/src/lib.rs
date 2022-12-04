//! Provides a high-level, back-end agnostic window API for [Wolf
//! Engine](https://docs.rs/wolf_engine/latest).
//!
//! # Examples
//!
//! ```
//! # use wolf_engine_window::prelude::*;
//! # 
//! # let window_backend = TestWindowBackend;
//! #
//! let window_settings = WindowSettings::new()
//!     .with_title("Example Window")
//!     .with_size((800, 600));
//! let window = window_backend.create_window(window_settings)
//!     .unwrap();
//! ```

mod window_settings;
pub use window_settings::*;

pub mod prelude {
    pub use super::*;
}

pub trait WindowBackend {
    type Window: Window;

    fn create_window(&mut self, settings: WindowSettings) -> Result<Self::Window, String>;
}

pub trait Window {}

#[doc(hidden)]
pub struct TestWindowBackend;

impl WindowBackend for TestWindowBackend {
    type Window = TestWindow;

    fn create_window(&mut self, _settings: WindowSettings) -> Result<Self::Window, String> {
        Ok(TestWindow)
    }
}

#[doc(hidden)]
pub struct TestWindow;

impl Window for TestWindow {}
