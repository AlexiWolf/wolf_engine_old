//! A simple, flexible, and easy to use game framework.
//!
//! ## Features
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
//! ## Concepts
//!
//! A small list of things you should understand when working with Wolf Engine.  Think of this as 
//! sort of a quick-start guide to ["groking"](https://en.wikipedia.org/wiki/Grok) the engine.
//!
//! ### Getting Started
//! 
//! - If you're just getting started, you should look at the 
//!   [`framework` module](wolf_engine_framework).  
//!
//! It includes a beginner-friendly guide to actually getting something happening on screen.  
//!
//! ### Going Deeper
//!
//! - The [`core` module](wolf_engine_core) has a nice overview of the base engine types.
//! - The [`events` module](events) contains the [`EventQueue` API](events::EventQueue), which is
//!   the base upon which the rest of the engine is built.
//!
//! ## Examples
//!
//! There are plenty of functional examples in the 
//! [examples/ directory](https://github.com/AlexiWolf/wolf_engine/tree/main/examples) in the repo.
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
