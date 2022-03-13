mod state_stack;
mod transition;

pub use state_stack::*;
pub use transition::*;

use crate::Context;

#[cfg(test)]
use mockall::automock;

/// A currently unused return type for [State]'s render method.
pub type RenderResult = ();

/// Provides a common mechanism for getting game logic / data to the engine.
///
/// A `State`, or "game state" is more or less the actual *game* part of your game.  Game
/// states allow you to store game data and implement your game's logic in way that makes
/// it easy to pass to send off to be run by the engine.
///
/// Wolf Engine games are made up of one or more States.  For simple games one state may
/// be fine, but more complex games may benefit from multiple states.  This allows you to
/// split your game into more manageable chunks.  For example: You may want to have a
/// `MainMenuState`, a `LevelState`, and a `PausedState` for your game.  Trying to shove
/// all of that game logic into a single state will quickly make your game unworkable.
///
/// Game states are managed by the engine's [StateStack].  The `update` method returns
/// a [Transition] type.  These transitions are used to tell the state machine what to do.
///
/// # Examples
///
/// ```
/// use wolf_engine::{Context, State, OptionalTransition, RenderResult};
///
/// struct MyGame {
///     number: u32,
/// }
///
/// impl State for MyGame {
///     fn update(&mut self, _context: &mut Context) -> OptionalTransition {
///         self.number += 1;
///         None // Don't transition, just keep running
///     }
///
///     fn render(&mut self, _context: &mut Context) -> RenderResult {
///         // Render logic
///     }
/// }
/// ```
#[cfg_attr(test, automock)]
pub trait State {

    /// Run one-time setup before the state is run.
    fn setup(&mut self, _context: &mut Context) {}
    
    /// Run one-time setup after the state is removed.
    fn shutdown(&mut self, _context: &mut Context) {}
    
    /// Runs any time the state is deactivated.  
    fn pause(&mut self, _context: &mut Context) {}
    
    /// Runs any time the state is reactivated.
    fn resume(&mut self, _context: &mut Context) {}

    /// Update the game state.
    fn update(&mut self, context: &mut Context) -> OptionalTransition;

    /// Update the game state in the background.
    fn background_update(&mut self, _context: &mut Context) {}

    /// Render the game state.
    fn render(&mut self, context: &mut Context) -> RenderResult;

    /// Render the game state in the background.
    fn background_render(&mut self, _context: &mut Context) -> RenderResult {}
}

/// A no-op state that will close immediately.
///
/// It's mostly here for doc-tests / examples to avoid rewriting the same empty state for
/// each one.
#[doc(hidden)]
pub struct EmptyState;

impl State for EmptyState {
    fn update(&mut self, _context: &mut Context) -> OptionalTransition {
        Some(Transition::Quit)
    }

    fn render(&mut self, _context: &mut Context) -> RenderResult {}
}
