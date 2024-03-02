//! Provides common tools, types, and functions used by the engine.
//!
//! # Getting Started
//!
//! When using the Core API, you are responsible for the main-loop, and responding to events.
//!
//! ```
//! # use wolf_engine_core as wolf_engine;
//! # use wolf_engine::prelude::*;
//! # use wolf_engine::resources::Resources;
//! #
//! # struct SomeResource;
//! #
//! pub fn main() {
//!     // First, initialize the EventLoop, and Context.
//!     // Resources, and other settings can also be set up from here.
//!     let (mut event_loop, mut context) = wolf_engine::init()
//!         // You can insert Resources, load plugins, ext. here.
//!         .with_resource(SomeResource)
//!         .build()
//!         .unwrap();
//!
//!     // The Event-Loop will continue to return events, every call, until a Quit event is sent,
//!     // only then, will the Event-Loop will return None.
//!     while let Some(event) = event_loop.next_event() {
//!         if let Ok(event) = event.downcast::<EngineEvent>() {
//!             process_event(*event, &mut context);
//!         }
//!     }
//! }
//!
//! pub fn process_event(event: EngineEvent, context: &mut Context) {
//!     match event {
//!         // Indicates there are no more events on the queue, or, essentially, the end of the
//!         // current frame.  
//!         EngineEvent::EventsCleared => {
//!             // You should put most of your game logic here.
//!
//!             // To close the game.
//! #           context.quit();
//!         }
//!         // Shut down the game.
//!         EngineEvent::Quit => println!("Quit event received.  Goodbye!"),
//!         _ => (),
//!     }
//! }
//! ```
//!
//! You can also look in the
//! [examples folder](https://github.com/AlexiWolf/wolf_engine/tree/main/examples) for additional
//! examples.

mod context;
pub use context::*;
pub mod engine_builder;
pub mod events;
pub mod plugins;

use engine_builder::state::Setup;
use engine_builder::EngineBuilder;
use events::EventLoop;

/// Represents the [`EventLoop`]-[`Context`] pair that makes up "the engine."
pub type Engine = (EventLoop, Context);

/// Provides a shared resource container which is thread-safe, and lock-free
///
/// Wolf Engine re-exports [`shared_resources`], see the original crate for details.
pub mod resources {
    pub use shared_resources::*;
}

#[doc(hidden)]
pub mod prelude {
    use super::*;

    pub use events::*;
    pub use super::{Context, Engine};
}

/// Initializes Wolf Engine using the [`EngineBuilder`].
pub fn init() -> EngineBuilder<Setup> {
    EngineBuilder::<Setup>::new()
}
