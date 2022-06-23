//! Wolf Engine is a game framework designed to be flexible an easy to work with.
//!
//! **Note:** Wolf Engine is still a W.I.P, and many features are still missing.  See the 
//! [README](https://github.com/Alexiwolf/wolf_engine#features) for more information.
//!
//! # Getting Started 
//! 
//! To get started with Wolf Engine, add the following to your `Cargo.toml`.
//!
//! ```text
//! [dependencies]
//! wolf_engine = "*"
//! ```
//! 
//! The [Engine] ships with sensible defaults to help you get up and running as quickly as possible.
//! To get started with the default settings, use [Engine::new()] or [Engine::default()], then 
//! run the engine by calling [Engine::run()], and passing your game's starting [State] to it.
//!
//! ```
//! use wolf_engine::*;
//! # 
//! # use wolf_engine::utils::EngineControls;
//!
//! # #[allow(clippy::needless_doctest_main)]
//! pub fn main() {
//!     Engine::new()
//!         .run(Box::from(MyGameState));
//! }
//!
//! pub struct MyGameState;
//!
//! impl State for MyGameState {
//!     fn update(&mut self, context: &mut Context) -> OptionalTransition {
//!         // Update your game here.
//! #       context.quit();
//!         None 
//!     }
//!
//!     fn render(&mut self, context: &mut Context) -> RenderResult {
//!         // Render your game here.
//!     }
//! }
//! ```
//!
//! # Important Concepts
//!
//! There are a few fundamental concepts you should understand when while working with Wolf Engine.
//!
//! ## Game States and the State Stack
//!
//! Wolf Engine games are implemented as one or more [State] objects.  A [State] holds all the 
//! logic and data for your game in a neat little package you can send to the [Engine].  When you
//! start the [Engine], you will provide it with your game [State] to run.
//!
//! The [Engine] does not run your game [State] directly.  Instead, the [State] is pushed onto the 
//! [StateStack], and is ran through it.  The [StateStack] stores and runs all active [State] 
//! objects for the [Engine].
//!
//! An example of a simple game state:
//!
//! ```
//! # use wolf_engine::*;
//! #
//! pub struct MyState {
//!     counter: u64, 
//! }
//!
//! impl State for MyState {
//!     fn update(&mut self, _context: &mut Context) -> OptionalTransition {
//!         counter += 1;
//!         None
//!     }
//!
//!     fn render(&mut self, _context: &mut Context) -> RenderResult {}
//! }
//! ```
//!
//! #### Multiple States 
//!
//! It is possible to have more than one [State] loaded and running at any given time.  The [Engine]
//! employs [StateStack] to allow multiple [State]s to be loaded at the same time.  The [StateStack] 
//! is controlled by the active [State] top of the stack) through [Transition]s returned by the 
//! [State::update()] method.
//!
//! When multiple [State]s are loaded, they all run together from bottom-to-top order.  A [State]
//! which is below another [State] on the stack is considered to be a "background" or "deactivated"
//! [State].  A background [State] will only have its [State::background_update()] and 
//! [State::background_render()] methods called. 
//! 
//! #### Changing States
//!
//! A [Transition] is ([Optionally](OptionalTransition)) returned by [State::update()] and is used 
//! to control the [StateStack] by pushing and popping [State]s on the [StateStack].
//!
//! ```
//! # use wolf_engine::*;
//! #
//! pub struct StateA {
//!     pub counter: u32,
//! }
//!
//! impl State for StateA {
//!     fn update(&mut self, _context: &mut Context) -> OptionalTransition {
//!         if self.counter < 10 {
//!             println!("Hello from State A!");
//!             // Increment the counter and push State B to the top of the stack.
//!             self.counter += 1;
//!             Some(Transition::Push(Box::from(StateB::new())))
//!         } else {
//!             // Once the counter reaches 10, pop all states off the stack.
//!             // An empty state stack will trigger an engine shutdown.
//!             Some(Transition::Clean)
//!         }
//!     }
//!
//!     fn render(&mut self, _context: &mut Context) -> RenderResult {}
//! }
//!
//! pub struct StateB {
//!     counter: u32,
//! }
//!
//! # impl StateB {
//! #   pub fn new() -> Self {
//! #       Self { counter: 0 }
//! #   }
//! # }
//! #
//! impl State for StateB {
//!     fn update(&mut self, _context: &mut Context) -> OptionalTransition {
//!         if self.counter < 3 {
//!             println!("Hello from State B!"); 
//!             // Increment the counter then return no transition.
//!             // The state will continue running as-is.
//!             self.counter += 1;
//!             None
//!         } else {
//!             // Once the counter reaches 3, pop this state off the stack.
//!             // This will return control back to the State A.
//!             Some(Transition::Pop)
//!         }
//!     }
//!
//!     fn render(&mut self, _context: &mut Context) -> RenderResult {}
//! }
//! ```
//! It is important to note that only the top [State] can control the [StateStack], as 
//! [State::update()] is the only method which can return a [Transition] to the [StateStack].  This
//! is the only way available to user code for sending commands to the [StateStack].  While this
//! may seem bit restrictive, it is done to keep state changes predictable.  When only the 
//! top [State] can change the [StateStack], there is no possibility of random parts of the code 
//! changing [State] in unpredictable ways.
//!
//! ## The Context Object
//!
//! ### Context Extension Traits
//!
//! ## The Engine Builder / Customizing the Engine 
//!
//! ## Engine Plugins / Extending the Engine 

mod core;

pub mod contexts;
pub mod event;
pub mod plugins;
pub mod schedulers;
pub mod utils;

#[cfg(feature = "logging")]
pub mod logging;

pub use crate::core::*;

use log::info;

pub(crate) fn log_startup_information() {
    let name = env!("CARGO_PKG_NAME");
    let version = env!("CARGO_PKG_VERSION");
    let homepage = env!("CARGO_PKG_HOMEPAGE");
    info!("Hello from {} v{} - {}", name, version, homepage);
}

pub(crate) fn log_shutdown() {
    info!("Engine has stopped.  Goodbye.")
}
