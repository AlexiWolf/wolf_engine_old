//! A simple, flexible, and easy to use game framework.
//!
//! # Features
//!
//! - `framework`: Enables the high-level, "batteries included" framework.
//! - `logging`: Enables the built-in logging framework.
//! - `serde`: Enables serde support for some types.
//! - `window`: Enables Wolf Engine's window API.
//!
//! # Getting Started
//!
//! To use the latest release version:
//!
//! ```ignore
//! [dependencies]
//! wolf_engine = "*"
//! ```
//!
//! To use the latest development version:
//!
//! ```ignore
//! wolf_engine = { git = "https://github.com/AlexiWolf/wolf_engine" }
//! ```
//!
//! # Usage
//!
//! If you're just getting started, or you're just making a game, you should use the
//! [Framework API](wolf_engine_framework).  
//!
//! If you're an advanced user, you can also use the [Core API](wolf_engine_core) directly.
//!
pub use wolf_engine_core::*;

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

    pub use wolf_engine_core::prelude::*;

    #[cfg(feature = "framework")]
    pub use framework::*;
}
