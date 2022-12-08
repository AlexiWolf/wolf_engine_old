//! Provides a high-level, back-end agnostic window API for [Wolf
//! Engine](https://docs.rs/wolf_engine/latest).

mod window;
pub use window::*;
mod window_dimensions;
pub use window_dimensions::*;
mod window_settings;
pub use window_settings::*;

#[doc(hidden)]
pub mod prelude {
    pub use super::*;
}
