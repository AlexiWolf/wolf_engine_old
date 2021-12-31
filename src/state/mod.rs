mod state_machine;
mod transition;

pub use state_machine::*;
pub use transition::*;

use crate::Context;

#[cfg(test)]
use mockall::automock;

/// A currently unused return type for [State]'s render method.
pub type RenderResult = ();

/// Provides a common mechanism for getting game logic / data to the engine.
///
/// A `State`, or "game state" is more or less the actual *game* part of your game.  
/// Game states allow you to store game data and implement your game's logic in way that 
/// makes it easy to pass to send off to be run by the engine.
///
/// Wolf Engine games are made up of one or more States.  For simple games one state may 
/// be fine, but more complex games may benefit from multiple states.  This allows you to 
/// split your game into more manageable chunks.  For example: You may want to have a 
/// `MainMenuState`, a `LevelState`, and a `PausedState` for your game.  Trying to shove
/// all of that game logic into a single state will quickly make your game unworkable.
/// 
/// Game states are managed by the engine's [StateMachine].  The `update` method returns
/// a [Transition] type.  These transitions are used to tell the state machine what to do.
#[cfg_attr(test, automock)]
pub trait State {
    fn update(&mut self, context: &mut Context) -> Transition;

    fn render(&mut self, context: &mut Context) -> RenderResult;
}
