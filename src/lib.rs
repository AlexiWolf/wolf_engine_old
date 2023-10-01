//! A simple, flexible game framework. 
//!
//! ## Usage 
//! 
//! - The [`framework` module](wolf_engine_framework), and [`core` module](wolf_engine_core) docs
//!   have some helpful quick-start examples you can follow to quickly jump-start your project.
//! - There are plenty of functional examples in the
//!   [examples folder](https://github.com/AlexiWolf/wolf_engine/tree/main/examples) in the
//!   repo.
//!
//! ## Crate Features
//!
//! - `framework`: Enable the high-level framework (enabled by default.)
//! - `logging`: Enable built-in logging implementation.
//! - `serde`: Enable [Serde](https://crates.io.crates/serde) support for some types.
//! - `window`: Enable Wolf Engine's high-level window API.

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

    #[cfg(feature = "framework")]
    pub use framework::*;
}
