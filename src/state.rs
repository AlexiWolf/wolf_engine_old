use crate::*;

#[cfg(test)]
use mockall::automock;

/// Indicates if a [Transition] should be performed.
pub type Transition = Option<TransitionType>;

/// Indicates the type of [Transition] the [StateStack](crate::StateStack) should
/// perform.
pub enum TransitionType {
    /// Push a new [State] to the top of the stack.
    Push(Box<dyn State>),

    /// Pop the active [State] off the stack and shut it down.
    Pop,

    /// Pop all [State]s off the stack, then push a new state.
    CleanPush(Box<dyn State>),

    /// Pop all [State]s off the stack, then shut down the engine.
    Clean,
}

/// Provides a way to package game data and logic to be run by the [Engine].
///
/// Wolf Engine games consist of one or more State objects.  Each implementing a specific
/// part of the game.  For example: It's useful to be able to break your game up into
/// manageable chunks such as a `MainMenuState`, a `LevelState`, and a `PausedState` where
/// each State does a single job.  This helps to break your game into manageable chunks.
///
/// By default, States are controlled by the [StateStack].  The [StateStack] allows States
/// to be stacked on top of each other and ran all together, resulting in a "layered"
/// behavior.  Active States can also control the [StateStack] by returning an
/// [Transition] from the [State::update()] method.  
///
/// See the [StateStack] docs for more information.
///
/// # Examples
///
/// ```
/// use wolf_engine::*;
///
/// struct MyGame {
///     number: u32,
/// }
///
/// impl State for MyGame {
///     fn update(&mut self, _context: &mut Context) -> Transition {
///         if self.number < 10 {
///             self.number += 1;
///             None // Don't transition, just keep running
///         } else {
///             // We've counted to 10, lets tell the engine to quit
///             Some(TransitionType::Clean)
///         }
///     }
///
///     fn render(&mut self, _context: &mut Context) {
///         // Render logic
///     }
/// }
/// ```
#[cfg_attr(test, automock)]
pub trait State {
    /// Run one-time setup before the state is run.
    ///
    /// There are no specific requirements for this method.  You may use it to do whatever
    /// your game needs.
    ///
    /// This method should be run only once throughout the life of the state object, and
    /// before any other method is run.
    fn setup(&mut self, _context: &mut Context) {}

    /// Run one-time cleanup after the state is removed.
    ///
    /// There are no specific requirements for this method.  You may use it to do whatever
    /// your game needs.
    ///
    /// This method should be run only once throughout the life of the state object, and
    /// it will always be the last method called before the state is dropped.
    fn shutdown(&mut self, _context: &mut Context) {}

    /// Pause the game state.
    ///
    /// There are no specific requirements for this method.  You may use it to do whatever
    /// your game needs.
    ///
    /// By default this method runs when:
    ///
    /// - The [StateStack] deactivates the state.
    /// - The application has gone out of focus (such as when the user switches apps on
    ///   mobile.)
    fn pause(&mut self, _context: &mut Context) {}

    /// Resume the game state.
    ///
    /// There are no specific requirements for this method.  You may use it to do whatever
    /// your game needs.
    ///
    /// By default this method runs when:
    ///
    /// - The [StateStack] reactivates the state.
    /// - The application has come back into focus (such as when the user switches apps on
    ///   mobile.)
    fn resume(&mut self, _context: &mut Context) {}

    /// Update the game state.
    ///
    /// By default, this method runs when:
    ///
    /// - The [Engine] requests a tick to run,
    /// - and the state is the topmost state on the [StateStack].
    fn update(&mut self, context: &mut Context) -> Transition;

    /// Update the game state in the background.
    ///
    /// Background updates are useful for allowing game states to continue running logic
    /// in the background, even when they are not the active state.  For example:
    /// Running timers, counters, or keeping the game running behind an inventory or pause
    /// menu.
    ///
    /// By default, this method runs when:
    ///
    /// - The [Engine] requests a tick to run,
    /// - and the state is not the topmost state on the [StateStack].
    fn background_update(&mut self, _context: &mut Context) {}

    /// Render the game state.
    ///
    /// By default, this method runs when:
    ///
    /// - The [Engine] requests a frame to render,
    /// - and the state is the topmost state on the [StateStack].
    fn render(&mut self, context: &mut Context);

    /// Render the game state in the background.
    ///
    /// Background renders are useful for allowing game states to continue rendering
    /// in the background, even when they are not the active state. For example:
    /// Continuing to render the game behind an inventory or pause menu.
    ///
    /// By default, this method runs when:
    ///
    /// - The [Engine] requests a frame to render,
    /// - and the state is not the topmost state on the [StateStack].
    fn background_render(&mut self, _context: &mut Context) {}
}

/// A no-op state that will close immediately.
///
/// It's mostly here for doc-tests / examples to avoid rewriting the same empty state for
/// each one.
#[doc(hidden)]
pub struct EmptyState;

impl State for EmptyState {
    fn update(&mut self, _context: &mut Context) -> Transition {
        Some(TransitionType::Clean)
    }

    fn render(&mut self, _context: &mut Context) {}
}
