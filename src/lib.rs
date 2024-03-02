//! A simple, flexible game framework.
//!
//! ## Getting Started
//!
//! - The [`framework` module](wolf_engine_framework), and [`core` module](wolf_engine_core) docs
//!   have some helpful quick-start examples you can follow to quickly jump-start your project.
//! - There are plenty of functional examples in the
//!   [examples folder](https://github.com/AlexiWolf/wolf_engine/tree/main/examples) in the
//!   repo.
//!
//! ## Crate Features
//!
//! - `framework`: Enables the high-level framework. (Default)
//! - `serde`: Enables [Serde](https://crates.io.crates/serde) support for some types.
//! - `window`: Enables Wolf Engine's high-level window API.

pub use wolf_engine_core as core;

#[cfg(feature = "framework")]
pub use wolf_engine_framework as framework;

#[cfg(feature = "window")]
pub use wolf_engine_window as window;

pub mod prelude {
    pub use wolf_engine_core::prelude::*;
}
