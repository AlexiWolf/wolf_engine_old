//! Wolf Engine is a game framework designed to be flexible an easy to work with.
//!
//! 
//! # Getting Started
//!
//! Wolf Engine ships with sensible defaults to help you jump-start projects as quickly 
//! as possible.  Using `Engine::new()` will initialize the engine with the default 
//! settings.
//!
//! ```
//! pub use wolf_engine::*;
//! 
//! pub fn main() {
//!     Engine::new()
//!         .run(Box::from(MyGameState::new()));
//! }
//!
//! pub struct MyGameState {
//!     number: usize,
//! }
//!
//! impl MyGameState {
//!     pub fn new() -> Self {
//!         Self {
//!             number: 0,
//!         }
//!     }
//! }
//!
//! impl State for MyGameState {
//!     fn update(&mut self, _context: &mut Context) -> OptionalTransition {
//!         // Update your game here.
//!         if self.number > 10 {
//!             self.number += 1;
//!             None
//!         } else {
//!             Some(Transition::Quit)
//!         }
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
