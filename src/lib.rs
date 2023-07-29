//! A simple, flexible, and easy to use game framework.
//!
//!
//! ## Concepts
//!
//! A small list of things you should understand when working with Wolf Engine.  Think of this as
//! sort of a quick-start guide to ["groking"](https://en.wikipedia.org/wiki/Grok) the engine.
//!
//! ### The Basics
//!
//! - If you're just getting started, you should look at the
//!   [`framework` module](wolf_engine_framework).  It includes a beginner-friendly guide to 
//!   actually getting something happening on screen.  
//! - The [`events` module](events) contains the [`EventQueue` API](events::EventQueue), which is
//!   the base upon which the rest of the engine is built.
//! - The [`core` module](wolf_engine_core) contains the base engine types.  It's very important 
//!   to understand if you plan to contribute to the engine, or build your own extensions.
//!
//! # Getting Started
//!
//! Add Wolf Engine to your project's `Cargo.toml` file.
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
//! ## Crate Features
//!
//! - `framework`: Enable the high-level framework (enabled by default.)
//! - `logging`: Enable built-in logging implementation.
//! - `serde`: Enable [Serde](https://crates.io.crates/serde) support for some types.
//! - `window`: Enable Wolf Engine's high-level window API.
//!
//! ## Usage Examples
//! - The [`framework` module](wolf_engine_framework), and [`core` module](wolf_engine_core) docs
//!   have some helpful quick-start examples you can follow to quickly jump-start your project.
//! - There are plenty of functional examples in the
//!   [examples/ directory](https://github.com/AlexiWolf/wolf_engine/tree/main/examples) in the
//!   repo.
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
