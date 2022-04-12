//! Wolf Engine is a game framework designed to be flexible an easy to work with.
//!
//! # Getting Started
//!
//! Wolf Engine ships with sensible defaults to help you jump-start projects as quickly 
//! as possible.  To get started with the default settings, use [Engine::new()] and 
//! provide your game's starting [State] to the [Engine::run()] method.
//!
//! ```
//! pub use wolf_engine::*;
//! 
//! pub fn main() {
//!     Engine::new()
//!         .run(Box::from(MyGameState));
//! }
//! 
//! pub struct MyGameState;
//!
//! impl State for MyGameState {
//!     fn update(&mut self, _context: &mut Context) -> OptionalTransition {
//!         // Update your game here.
//! #       return Some(Transition::Quit);
//!         None
//!     }
//!
//!     fn render(&mut self, _context: &mut Context) -> RenderResult {
//!         // Render your game here.
//!     }
//! }
//! ```
//!
//! # Context Data
//!
//! [Engine] data is stored in the [Context].  The [Context] is a dynamic storage 
//! container which allows us to add new data to the [Engine] at run-time.
//!
//! If you have access to the [Context], you can request access to stored data by it's 
//! type.  For [Subcontext]s, Rusts normal borrowing rules still apply, but they are 
//! checked at run-time rather than at compile-time.  This is done to help avoid issues
//! with the borrow checker when borrowing multiple [Subcontext]s.
//!
//! ```
//!# pub use wolf_engine::*;
//!# 
//!# pub fn main() {
//!#     Engine::new()
//!#         .run(Box::from(MyGameState));
//!# }
//!# 
//!# pub struct MyGameState;
//!#
//!# impl State for MyGameState {
//! fn update(&mut self, context: &mut Context) -> OptionalTransition {
//!     if let Some(Ok(subcontext)) = context.try_borrow::<ExampleContext>() {
//!         log::info!("{}", subcontext.message);     
//!         // This subcontext must go out of scope before we can borrow mutably.
//!     }
//!     if let Some(Ok(mut subcontext)) = context.try_borrow_mut::<ExampleContext>() {
//!         // If anything else is borrowing the subcontext, we will get an error.
//!         subcontext.message = "New Message".to_string();
//!         log::info!("{}", subcontext.message);
//!     }
//!#    return Some(Transition::Quit);
//!     None
//! }
//!#
//!#     fn render(&mut self, _context: &mut Context) -> RenderResult {}
//!# }
//!# 
//!# pub struct ExampleContext {
//!#    pub message: String,  
//!# } 
//!#
//!# impl Subcontext for ExampleContext {}
//! ```
//!
//! It is best to use [Context::try_borrow()] and [Context::try_borrow_mut()] instead of 
//! [Context::borrow()] and [Context::borrow_mut()], as the non-try methods will panic if 
//! the borrowing rules are broken.
//!
//! ## Functions Using the Context 
//!
//! A common pattern for Wolf Engine is passing the [Context] or specific [Subcontext] 
//! objects to functions.  Because the [Context] grants access to all [Engine] data, 
//! functions can use it to work on the current instance of the [Engine].
//! 
//! ```
//!# use wolf_engine::*;
//!#
//!# let context = &Context::new();
//!# fn some_function(context: &Context) {}
//! some_function(context);
//! ```
//!
//! Implementing new functions using this pattern is also easy.
//!
//! ```
//! # use wolf_engine::*;
//! #
//! pub fn my_function(context: &Context) {
//!     // Do something cool.
//! }
//! ```
//!
//! # Advanced Usage
//!
//! More complete examples can be found in the 
//! [Examples Folder](https://github.com/AlexiWolf/wolf_engine/tree/main/examples).
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
