//! A simple, flexible, and easy to use game framework.
//!
//! Wolf Engine is devided into two major components:
//!
//! - The Framework: `wolf_engine_framework`.
//! - The Core API: [`wolf_engine_core`].
//!
//! Refer to each module's documentation for a quickstart guide, and usage examples.  In most
//! cases, game projects will want to start with the Framework.  The Core API is mostly used to
//! build engine extensions, and custom frameworks.
//!
//! # Features
//!
//! - `framework`: Enables the `wolf_engine_framework` module (default.)
//! - `logging`: Enables the built-in logging framework.

pub use wolf_engine_core::*;

#[cfg(feature = "framework")]
pub use wolf_engine_framework as framework;

#[cfg(feature = "logging")]
pub use wolf_engine_core::logging;

#[doc(hidden)]
pub mod prelude {
    pub use super::*;

    pub use wolf_engine_core::prelude::*;

    #[cfg(feature = "framework")]
    pub use framework::*;
}
