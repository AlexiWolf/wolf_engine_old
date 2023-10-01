//! Provides common tools, types, and functions used by the engine.
//!
//! # Getting Started
//!
//! When using the Core API, you are responsible for the main-loop, and responding to events.
//!
//! ```
//! # use wolf_engine_core as wolf_engine;
//! use wolf_engine::prelude::*;
//! #
//! # struct SomeResource;
//! #
//! # #[wolf_engine::ecs::system]
//! # fn example() {}
//!
//! pub fn main() {
//!     // Start by initializing the engine's Event-Loop, and Context.
//!     let (mut event_loop, mut context) = wolf_engine::init::<()>()
//!         .with_resources(|resources| {
//!             // Here is where you add Resources, or custom data to the engine.
//!             // These resources are available to systems, and from the Context at run-time.
//!             resources.add_resource(SomeResource);
//!         })
//!         .with_update_schedule(|schedule| {
//!             // Here is where you build up the Update schedule.
//!             // This schedule is ran when you call Context::update().
//!             schedule.add_system(example_system());
//!         })
//!         .with_render_schedule(|schedule| {
//!             // Here is where you build up the Render schedule.
//!             // This schedule is ran when you call Context::render().
//!             schedule.add_system(example_system());
//!         })
//!         .build();
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
//!             context.update();
//!             context.render();
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
use std::marker::PhantomData;

pub use context::*;
mod event_loop;
pub use event_loop::*;

/// Provides an Entity-Component-System based on [Legion](::legion).
pub mod ecs {
    pub use legion::*;
    
    /// A, more clearly-named, alias to [`systems::Builder`].
    pub type ScheduleBuidler = legion::systems::Builder;
    
    /// Provides a builder-pattern for creating [`Resources`].
    #[derive(Default)]
    pub struct ResourcesBuilder {
        resources: Resources,
    }

    impl ResourcesBuilder {
        /// Inserts the provide instance of `T` into the [`Resources`].
        /// 
        /// If the provided type has previously been added, the existing instance is silently
        /// overwritten.
        ///
        /// This function is functionally-identical to calling [`Resources::insert()`].
        pub fn add_resource<T: systems::Resource + 'static>(&mut self, resource: T) -> &mut Self {
            self.resources.insert(resource);
            self
        }
        
        /// Consumes the builder, and returns the [`Resources`] from it.
        pub fn build(self) -> Resources {
            self.resources
        }
    }
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
    resources: ResourcesBuilder,
    update_schedule_builder: ScheduleBuidler,
    render_schedule_builder: ScheduleBuidler,
    _event_type: PhantomData<E>,
}

impl<E: UserEvent> EngineBuidler<E> {
    pub(crate) fn new() -> Self {
        Self {
            resources: ResourcesBuilder::default(),
            update_schedule_builder: Schedule::builder(),
            render_schedule_builder: Schedule::builder(),
            _event_type: PhantomData::default(),
        }
    }

    pub fn with_resources(mut self, function: fn(&mut ResourcesBuilder)) -> Self {
        (function)(&mut self.resources);
        self
    }

    pub fn with_update_schedule(mut self, function: fn(&mut ScheduleBuidler)) -> Self {
        (function)(&mut self.update_schedule_builder);
        self
    }

    pub fn with_render_schedule(mut self, function: fn(&mut ScheduleBuidler)) -> Self {
        (function)(&mut self.render_schedule_builder);
        self
    }

    pub fn build(mut self) -> Engine<E> {
        let event_loop = EventLoop::new();
        let context = Context::<E>::builder()
            .with_resources(self.resources.build())
            .with_update_schedule(self.update_schedule_builder.build())
            .with_render_schedule(self.render_schedule_builder.build())
            .build(&event_loop);
        (event_loop, context)
    }
}

/// Initializes a new instance of the [`EventLoop`], and its associated [`Context`], with the
/// provided data.
pub fn init<E: UserEvent>() -> EngineBuidler<E> {
    EngineBuidler::new()
}

#[cfg(test)]
mod init_tests {
    #[test]
    fn should_use_builder_pattern() {
        let (_event_loop, _context) = crate::init::<()>()
            .with_resources(|resources| {
                resources.add_resource(0).add_resource(true);
            })
            .with_update_schedule(|schedule| {
                schedule
                    .add_system(test_system())
                    .add_thread_local(test_system())
                    .flush()
                    .add_thread_local_fn(|_, _| {});
            })
            .with_render_schedule(|schedule| {
                schedule
                    .add_system(test_system())
                    .add_thread_local(test_system())
                    .flush()
                    .add_thread_local_fn(|_, _| {});
            })
            .build();
    }

    #[crate::ecs::system]
    fn test() {
        println!("Hello, world!");
    }
}
