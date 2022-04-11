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
