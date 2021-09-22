mod fixed_update_game_loop;

use crate::core::Context;

pub use fixed_update_game_loop::*;

/// Indicates the status of the GameLoop. For now, this doesn't do anything.
pub type LoopResult = ();

/// Represents the number of ticks a [GameLoop] has performed.
pub type Ticks = u64;

/// Represents the number of frames a [GameLoop] has rendered.
pub type Frames = u64;

/// Controls how the game is run.
///
/// The `GameLoop` is the core control structure Wolf Engine, as it directly controls how the game
/// is run.  It is primarily responsible for:
///
/// - Updating the game state.
/// - Rendering the current frame.
/// - Tracking basic information about itself, such as the number of *ticks* performed, and
///   *frames* it's rendered.  TODO: As well as tick and frame timing information.
///
/// Wolf Engine's default `GameLoop` is the
/// [FixedUpdateGameLoop](fixed_update_game_loop::FixedUpdateGameLoop).  See its documentation for
/// usage information.
///
/// Different `GameLoop`s may operate differently, so you should refer to implementation
/// documentation for specific details,
///
/// # Implementing a Game Loop
///
/// Wolf Engine also fully supports using a custom `GameLoop`.  Simply implement this trait.
///
/// ```
/// use wolf_engine::core::{GameLoop, Context, LoopResult};
/// # use wolf_engine::core::{Frames, Ticks};
///
/// pub struct MyGameLoop;
///
/// impl GameLoop for MyGameLoop {
///     fn update<F>(&mut self, context: &mut Context, mut update_function: F) -> LoopResult
///     where F: FnMut(&mut Context) -> LoopResult {
///         update_function(context)
///     }
///
///     fn render<F>(&mut self, context: &mut Context, mut render_function: F) -> LoopResult
///     where F: FnMut(&mut Context) -> LoopResult {
///         render_function(context)
///     }
///#
///#     fn ticks(&self) -> Ticks {
///#         0
///#     }
///#
///#     fn frames(&self) -> Frames {
///#         0
///#     }
/// }
/// ```
///
/// ## Updating
///
/// At it's most basic, the `update` function just calls the game's update function and returns
/// the result from it.  Depending on the implementation, different update strategies may be used
/// and / or additional timing controls may be added.
///
/// ### Update != Tick
///
/// In the context of a `GameLoop`, an *update* refers to a single call to the `update` function,
/// while a *tick* refers to a single call to the game's `update` function.  There is no expected
/// number of *ticks* for a single *update*.  An *update* may trigger 0, 1, or any other number of
/// *ticks* depending on the implementation.
///
/// ## Rendering
///
/// The most basic `render` implementation simply calls the game's render function and returns the
/// result from it.  Additional timing controls (Vsync, frame-limiting, ext.), or frame
/// interpolation may be added.  Generally, a single call to `render` should render a single frame,
/// but this is not a hard requirement.
///
pub trait GameLoop {

    /// Update the game state.
    fn update<F>(&mut self, context: &mut Context, update_function: F) -> LoopResult
    where
        F: FnMut(&mut Context) -> LoopResult;

    /// Render the game state.
    fn render<F>(&mut self, context: &mut Context, render_function: F) -> LoopResult
    where
        F: FnMut(&mut Context) -> LoopResult;

    /// Access the number of ticks that have been performed.
    fn ticks(&self) -> Ticks;

    /// Access the number of frames tha have been rendered.
    fn frames(&self) -> Frames;
}
