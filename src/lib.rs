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
//! ## Basic Usage
//!  
//! ```
//! use wolf_engine::prelude::*;
//!
//! let mut engine = Engine::new();
//!
//! // The Engine will continue to return events until it quits.
//! while let Some(event) = engine.next_event() {
//!     match event {
//!         Event::Quit => {
//!             // Shut down the game.
//!         },
//!         Event::Update => {
//!             // Update the game.
//!
//!             // To shut down the Engine, you must send a quit event.
//!             engine.send_event(Event::Quit);
//!         },
//!         Event::Render => {
//!             // Render the game.
//!         },
//!         Event::EventsCleared => {
//!             // Note: The engine will not emit Update / Render events on its own.
//!             //       You are expected to do this yourself.
//!             engine.send_event(Event::Update);
//!             engine.send_event(Event::Render);
//!         }
//!         _ => (),
//!     }
//! }
//! ```
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
