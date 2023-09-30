//! Provides common tools, types, and functions for the engine.
//!
//! The Core API provides all the parts likely to be (re)used by other parts of the engine.  It
//! is mostly intended for those building, or making extensions to Wolf Engine, but there are some
//! tools for end-users as well.
//!
//! # Getting Started
//!
//! While it's possible to build games using the `core` module alone, this isn't recommended
//! unless you *really* know what you're doing, or you want to build your own, game-specific,
//! engine.
//!
//! The core module really doesn't do a lot on it's own.  It's closer to a collection of basic
//! tools than it is an actual game framework.  As such, you're expected to write your own
//! main-loop, and respond to events entirely on your own.
//!
//! Here's an example of a basic main-loop:
//!
//! ```
//! # use wolf_engine_core as wolf_engine;
//! use wolf_engine::prelude::*;
//!
//! pub fn main() {
//!     // Start by initializing the engine's Event-Loop, and Context.
//!     let (mut event_loop, mut context) = wolf_engine::init::<()>().build();
//!     
//!     // The Event-Loop will continue to return events, every call, until a Quit event is sent,
//!     // only then, will the Event-Loop will return None.
//!     while let Some(event) = event_loop.next_event() {
//!         process_event(event, &mut context);
//!     }
//! }
//!
//! pub fn process_event(event: Event<()>, context: &mut Context<()>) {
//!     match event {
//!         // Indicates there are no more events on the queue, or, essentially, the end of the
//!         // current frame.  
//!         Event::EventsCleared => {
//!             // You should put most of your game logic here.
//!             context.quit();
//!         }
//!         // Shut down the game.
//!         Event::Quit => println!("Quit event received.  Goodbye!"),
//!         _ => (),
//!     }
//! }
//! ```
//!
//! You can use this example as a jumping-off point for your game.  Most of Wolf Engine's libraries
//! are built against `core`, so you can very likely pull in other modules and start using them
//! without to much trouble.
//!
//! You can also look in the
//! [examples folder](https://github.com/AlexiWolf/wolf_engine/tree/main/examples) for additional
//! examples.

mod context;
use std::marker::PhantomData;

pub use context::*;
mod event_loop;
pub use event_loop::*;

pub mod ecs {
    pub use legion::*;
}
pub mod events;

#[cfg(feature = "logging")]
pub mod logging;

#[doc(hidden)]
pub mod prelude {
    pub use super::*;
    pub use events::*;
}

use ecs::*;
use events::UserEvent;

/// Represents the [`EventLoop`]-[`Context`] pair that makes up "the engine."
pub type Engine<E> = (EventLoop<E>, Context<E>);

pub struct EngineBuidler<E: UserEvent> {
    resources: Resources,
    schedule_builder: ecs::systems::Builder,
    _event_type: PhantomData<E>,
}

impl<E: UserEvent> EngineBuidler<E> {
    pub(crate) fn new() -> Self {
        Self {
            resources: Resources::default(),
            schedule_builder: Schedule::builder(),
            _event_type: PhantomData::default(),
        }
    }

    pub fn with_resources(mut self, function: fn(&mut Resources)) -> Self {
        (function)(&mut self.resources);
        self
    }

    pub fn with_systems(mut self, function: fn(&mut ecs::systems::Builder)) -> Self {
        (function)(&mut self.schedule_builder);
        self
    }

    pub fn build(mut self) -> Engine<E> {
        let event_loop = EventLoop::new();
        let context = Context::<E>::builder()
            .with_resources(self.resources)
            .with_schedule(self.schedule_builder.build())
            .build(&event_loop);
        (event_loop, context)
    }
}

/// Initializes a new instance of the [`EventLoop`], and its associated [`Context`], with the
/// provided data.
///
/// #  Examples
///
/// ```
/// # use wolf_engine_core as wolf_engine;
/// #
/// // The prelude brings in commonly needed types, and traits.
/// use wolf_engine::prelude::*;
///
/// // Start by initializing the EventLoop, and Context.
/// // In this case, we are not using any Context data, so `()` is used.
/// let (mut event_loop, mut context) = wolf_engine::init::<()>().build();
///
/// // Then, you can use the EventLoop to run your game's main-loop.
/// while let Some(event) = event_loop.next_event() {
///     // Do something cool!
/// #   break;
/// }
/// ```
///
/// ## Custom Context Data
///  
/// The [`Context`] documentation has more detailed information about context data.  It's a good
/// place to start, if you're interested in customizing the engine.
///
/// ```
/// # use wolf_engine_core as wolf_engine;
/// # pub struct SomeCustomDataType {};
/// #
/// # use wolf_engine::prelude::*;
/// let (mut event_loop, mut context) = wolf_engine::init::<()>().build();
/// ```
pub fn init<E: UserEvent>() -> EngineBuidler<E> {
    EngineBuidler::new()
}

#[cfg(test)]
mod init_tests {
    #[test]
    fn should_use_builder_pattern() {
        let (_event_loop, _context) = crate::init::<()>()
            .with_resources(|resources| {
                resources.insert(0);
            })
            .with_systems(|systems| {
                systems
                    .add_thread_local_fn(|_, _| {})
                    .add_thread_local_fn(|_, _| {});
            })
            .build();
    }
}
