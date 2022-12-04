//! Provides a high-level, back-end agnostic window API for [Wolf
//! Engine](https://docs.rs/wolf_engine/latest).
//!
//! # Examples
//!
//! ```
//! # let window_backend = TestWindowBackend::new();
//! #
//! let window_settings = WindowSettings::new()
//!     .with_title("Example Window")
//!     .with_size((800, 600));
//! let window = window_backend.create_window(window_settings)
//!     .unwrap();
//! ```

mod window_settings;
pub use window_settings::*;

