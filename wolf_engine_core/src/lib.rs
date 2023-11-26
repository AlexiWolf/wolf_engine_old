//! Provides common tools, types, and functions used by the engine.
//!
//! # Getting Started
//!
//! When using the Core API, you are responsible for the main-loop, and responding to events.
//!
//! ```
//! # use wolf_engine_core as wolf_engine;
//! # use wolf_engine::prelude::*;
//! # use wolf_engine::ecs::{Schedule, ResourcesBuilder};
//! #
//! # struct SomeResource;
//! #
//! # #[legion::system]
//! # fn example() {}
//!
//! pub fn main() {
//!     // Start by setting up Resources, or custom data for the engine.
//!     // These resources are available to systems, and from the Context at run-time.
//!     // This step is optional.
//!     let mut resources = ResourcesBuilder::default();
//!     resources.add_resource(SomeResource);
//!
//!     // Then initalize the EventLoop, and Context.
//!     // Resources, and other settings can also be set up from here.
//!     let (mut event_loop, mut context) = wolf_engine::init::<()>()
//!         .with_resources(resources)
//!         .build();
//!
//!     let mut schedule = Schedule::builder()
//!         .add_system(example_system())
//!         .build();
//!     
//!     // The Event-Loop will continue to return events, every call, until a Quit event is sent,
//!     // only then, will the Event-Loop will return None.
//!     while let Some(event) = event_loop.next_event() {
//!         process_event(event, &mut context, &mut schedule);
//!     }
//! }
//!
//! pub fn process_event(event: Event<()>, context: &mut Context<()>, schedule: &mut Schedule) {
//!     match event {
//!         // Indicates there are no more events on the queue, or, essentially, the end of the
//!         // current frame.  
//!         Event::EventsCleared => {
//!             // You should put most of your game logic here.
//!
//!             // You can run ECS schedules through the Context.
//!             context.run_schedule(schedule);
//!
//!             // To close the game.
//! #           context.quit();
//!         }
//!         // Shut down the game.
//!         Event::Quit => println!("Quit event received.  Goodbye!"),
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

pub mod ecs;
pub mod events;

#[cfg(feature = "logging")]
pub mod logging;

#[doc(hidden)]
pub mod prelude {
    pub use super::*;
    pub use events::*;
}

use crate::prelude::UserEvent;

/// Creates a new [`EngineBuilder`] to set up the [`Engine`].
pub fn init<E: UserEvent>() -> EngineBuilder<E> {
    EngineBuilder::new()
}

#[cfg(test)]
mod init_tests {
    use crate::{ecs::ResourcesBuilder, events::MainEventSender};

    #[test]
    fn should_add_resources() {
        let mut resources = ResourcesBuilder::default();
        resources.add_resource(0).add_resource(true);

        let (_event_loop, context) = crate::init::<()>().with_resources(resources).build();

        assert!(
            context.resources().get::<i32>().is_some(),
            "The resources were not used"
        );
    }

    #[test]
    fn should_add_event_sender_resource_by_default() {
        let (_event_loop, context) = crate::init::<()>().build();
        let _event_sender = context
            .resources()
            .get_mut::<MainEventSender<()>>()
            .expect("No event sender was added.");
    }
}
