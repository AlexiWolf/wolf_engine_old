//! Provides common tools, types, and functions for the engine.
//!
//! The Core API provides all the parts likely to be (re)used by other parts of the engine.  It
//! is mostly intended for those building, or making extensions to Wolf Engine, but there are some
//! tools for end-users as well.
//!
//! # Getting Started
//!
//! ```
//! # use wolf_engine_core as wolf_engine;
//! use wolf_engine::prelude::*;
//! 
//! pub struct GameData {
//!     pub number: i32,
//! }
//! 
//! pub fn main() {
//!     let (mut event_loop, mut context) = wolf_engine::init(GameData { number: 0 });
//! 
//!     while let Some(event) = event_loop.next_event() {
//!         process_event(event, &mut context);
//!     }
//! }
//! 
//! pub fn process_event(event: Event, context: &mut Context<GameData>) {
//!     match event {
//!         // Shut down the game.
//!         Event::Quit => println!("Quit event received.  Goodbye!"),
//!         // Update the game.
//!         Event::Update => {
//!             if context.data.number == 3 {
//!                 // To shut down the Engine, you must send a quit event.
//!                 context.quit();
//!             } else {
//!                 context.data.number += 1;
//!             }
//!         }
//!         Event::Render => println!("{}", context.data.number),
//!         Event::EventsCleared => {
//!             // Note: The engine will not emit Update / Render events on it's own.
//!             //       You are expected to do this yourself.
//!             context.update();
//!             context.render();
//!         }
//!         _ => (),
//!     }
//! }
//! ```

mod context;
pub use context::*;
mod event_loop;
pub use event_loop::*;

pub mod events;

#[cfg(feature = "logging")]
pub mod logging;

#[doc(hidden)]
pub mod prelude {
    pub use super::*;
    pub use events::*;
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
/// let (mut event_loop, mut context) = wolf_engine::init(());
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
/// let (mut event_loop, mut context) = wolf_engine::init(SomeCustomDataType {});
/// ```
pub fn init<D>(data: D) -> (EventLoop, Context<D>) {
    let event_loop = EventLoop::new();
    let context = Context::new(&event_loop, data);
    (event_loop, context)
}
