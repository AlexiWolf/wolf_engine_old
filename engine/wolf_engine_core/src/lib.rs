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
//!     // Start by setting up Resources, or custom data for the engine.
//!     // These resources are available to systems, and from the Context at run-time.
//!     // This step is optional.
//!     let mut resources = Resources::default();
//!     resources.insert(SomeResource);
//!
//!     // Then initalize the EventLoop, and Context.
//!     // Resources, and other settings can also be set up from here.
//!     let (mut event_loop, mut context) = wolf_engine::init::<()>()
//!         .with_resources(resources)
//!         .build();
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
//! pub fn process_event(event: EngineEvent, context: &mut Context<()>) {
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
mod event_loop;
pub use event_loop::*;
mod engine_builder;
pub use engine_builder::*;

pub mod events;

/// Provides a shared resource container which is thread-safe, and lock-free
///
/// Wolf Engine re-exports [`shared_resources`], see the original crate for details.
pub mod resources {
    pub use shared_resources::*;
}

#[cfg(feature = "logging")]
pub mod logging;

#[doc(hidden)]
pub mod prelude {
    pub use super::*;
    pub use events::*;
}

use crate::prelude::UserEvent;

/// Initializes Wolf Engine using the [`EngineBuilder`].
pub fn init<E: UserEvent>() -> EngineBuilder<E> {
    EngineBuilder::new()
}

#[cfg(test)]
mod init_tests {
    use crate::events::MainEventSender;
    use crate::resources::Resources;

    #[test]
    fn should_add_resources() {
        let mut resources = Resources::default();
        resources.insert(0);

        let (_event_loop, context) = crate::init::<()>().with_resources(resources).build();

        assert!(
            context.resources().get::<i32>().is_ok(),
            "The resources were not used"
        );
    }

    #[test]
    fn should_add_event_sender_resource_by_default() {
        let (_event_loop, context) = crate::init::<()>().build();
        let _event_sender = context
            .resources()
            .get_mut::<MainEventSender>()
            .expect("No event sender was added.");
    }
}
