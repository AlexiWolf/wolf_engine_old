//! Provides flexible update and frame timing controls for [WolfEngine](crate::WolfEngine).

mod fixed_update_game_loop;

pub use fixed_update_game_loop::*;

use crate::{Context, State};
use std::fmt::Display;

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
///   *frames* it's rendered.
///
/// Wolf Engine's default `GameLoop` is the
/// [FixedUpdateGameLoop](fixed_update_game_loop::FixedUpdateGameLoop).  See its documentation for
/// usage information.
///
/// Different `GameLoop`s may operate differently, so you should refer to implementation
/// documentation for specific details,
///
/// ## Updating
///
/// At it's most basic, the `update` function just calls the game's update function and returns
/// the result from it.  Depending on the implementation, different update strategies may be used
/// and / or additional timing controls may be added.
///
/// Ticks vs Updates:
///
/// - An *update* refers to a single call to the game loop's `update` method.
/// - A *tick* refers to a single step of the game's state.
/// - There may be 0, 1, or any other number of *ticks* in an *update*.
///
/// ## Rendering
///
/// The most basic `render` implementation simply calls the game's render function and
/// returns the result from it.  Additional timing controls (Vsync, frame-limiting, ext.),
/// or frame interpolation may be added.  Generally, a single call to `render` should
/// render a single frame.
///
/// # Custom Game Loops
///
/// Wolf Engine also fully supports using a custom `GameLoop`.  Simply implement this
/// trait.
///
/// ```
/// use wolf_engine::{State, Context, game_loop::{GameLoop, LoopResult}};
/// # use std::fmt::{Formatter, Display};
///
/// pub struct MyGameLoop;
///
/// impl GameLoop for MyGameLoop {
///
///     fn update(&mut self, context: &mut Context, state: &mut dyn State) -> LoopResult {
///         state.update(context);
///     }
///
///     fn render(&mut self, context: &mut Context, state: &mut dyn State) -> LoopResult {
///         state.render(context);
///     }
/// }
/// #
/// # impl Display for MyGameLoop {
/// #     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
/// #         write!(f, "")
/// #     }
/// # }
/// ```
///
/// You can use a custom [GameLoop](crate::game_loop::GameLoop) implementation by using
/// the `WolfEngineBuilder::with_custom_game_loop()` method.
///
/// ```
/// # use wolf_engine::{
/// #    ContextBuilder, WolfEngineBuilder, game_loop::FixedUpdateGameLoopBuilder
/// # };
/// #
/// # let (context, event_loop) = ContextBuilder::new()
/// #    .build();
/// #
/// # // Using fixed game loop for the example because the actual loop is unimportant.
/// # // Any GameLoop can be provided here and it will work just the same.
/// # let custom_game_loop = FixedUpdateGameLoopBuilder::new()
/// #   .build();
/// #
/// let engine = WolfEngineBuilder::with_custom_game_loop(custom_game_loop)
///     .build(context);
/// ```
pub trait GameLoop: Display {
    /// Update the game state.
    fn update(&mut self, context: &mut Context, state: &mut dyn State) -> LoopResult;

    /// Render the game state.
    fn render(&mut self, context: &mut Context, state: &mut dyn State) -> LoopResult;
}
