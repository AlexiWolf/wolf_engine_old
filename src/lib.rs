//! Wolf Engine is a game framework designed to be flexible an easy to work with.
//!
//! # Getting Started
//!
//! Wolf Engine ships with sensible defaults to help you jump-start projects as quickly 
//! as possible.  To get started with the default settings, use [Engine::new()] and 
//! provide your game's starting [State] to the [Engine::run()] method.
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
//!
//! # Game States
//!
//! Wolf Engine games are implemented as a collection of [State] objects.  Each [State] 
//! holds some data and logic for a specific part of the game.  The main use for [State]s
//! is to help you split your game into manageable pieces.
//!
//! For example, you may have a game with several parts: 
//!
//! - A main menu screen. 
//! - A level / game-play screen.
//! - A "game over" screen.
//!
//! Instead of trying to shove all these behaviors into a single place, you could divide 
//! your game into several states instead:
//!
//! - `MainMenuState`: Displays the game's logo and has a play button.
//! - `GameplayState`: Runs the main game logic.  
//! - `GameOverState`: Shows the player's final score, and prompts them to play again.
//!
//! ## The State Stack
//!
//! The [StateStack] is the main organizational component of the [Engine].  It allows for 
//! [State] objects to be stacked on top of each other, and allows for "layered" game 
//! logic.  The state on the top of the stack is designated the `active` [State], and all 
//! other [State]s are designated as `inactive.`  The active [State] assumes control, 
//! while inactive [State]s continue to run the background.
//!
//! A good example of why this system is useful is when implementing a pause menu for your
//! game.  When the player pauses the game:  
//!
//! 1. The `PausedState` is pushed to the stack, and it takes over as the `active` state.
//! 2. When the player is done in the pause menu, the `PausedState` is popped off the 
//!    stack, and the `GameState` becomes `active` again and is resumed. 
//!
//! A very useful effect of layered game logic is it's possible for inactive [State]s to 
//! continue running in the background.  For example, if your game is multi-player, the 
//! world can continue running even though the client is "paused."  In other cases, such
//! as if your pause menu is transparent, you may want to stop game logic from updating,
//! but continue running animations and effects in the background.
//!
//! ## State Transitions
//!
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
