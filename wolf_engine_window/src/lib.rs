//! Provides a high-level, back-end agnostic window API for [Wolf
//! Engine](https://docs.rs/wolf_engine/latest).
//!
//! See [wolf_engine::window](https://docs.rs/wolf_engine/latest/wolf_engine/window/index.html/)
//! for more details.

mod window;
pub use window::*;
mod window_dimensions;
pub use window_dimensions::*;
mod window_id;
pub use window_id::*;
mod window_settings;
pub use window_settings::*;

#[doc(hidden)]
pub mod prelude {
    pub use super::*;
}
