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
//! - `logging`: Enables built-in logging implementation.
//! - `serde`: Enables [Serde](https://crates.io.crates/serde) support for some types.
//! - `window`: Enables Wolf Engine's high-level window API.

pub use wolf_engine_core::prelude::*;

#[cfg(feature = "framework")]
pub mod framework {
    //! Provides a high-level, "batteries-included" framework.
    pub use wolf_engine_framework::*;
}

#[cfg(feature = "logging")]
pub use wolf_engine_core::logging;

#[cfg(feature = "window")]
pub mod window {
    //! Provides a high-level, back-end agnostic window API.
    pub use wolf_engine_window::*;
}

#[doc(hidden)]
pub mod prelude {
    pub use super::*;
}
