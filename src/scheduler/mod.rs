//! Provides flexible update and frame timing controls for [WolfEngine](crate::WolfEngine).

mod fixed_update_scheduler;

pub use fixed_update_scheduler::*;

use crate::{Context, State};
use std::fmt::Display;

/// Represents the number of ticks a [Scheduler] has performed.
pub type Ticks = u64;

/// Represents the number of frames a [Scheduler] has rendered.
pub type Frames = u64;

/// Controls how the game is run.
///
/// The `Scheduler` is the core control structure Wolf Engine, as it directly controls how the game
/// is run.  It is primarily responsible for:
///
/// - Updating the game state.
/// - Rendering the current frame.
/// - Tracking basic information about itself, such as the number of *ticks* performed, and
///   *frames* it's rendered.
///
/// Wolf Engine's default `Scheduler` is the
/// [FixedUpdateScheduler](fixed_update_scheduler::FixedUpdateScheduler).  See its documentation for
/// usage information.
///
/// Different `Scheduler`s may operate differently, so you should refer to implementation
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
/// Wolf Engine also fully supports using a custom `Scheduler`.  Simply implement this
/// trait.
///
/// ```
/// use wolf_engine::{State, Context, scheduler::{Scheduler, LoopResult}};
/// # use std::fmt::{Formatter, Display};
///
/// pub struct MyScheduler;
///
/// impl Scheduler for MyScheduler {
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
/// # impl Display for MyScheduler {
/// #     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
/// #         write!(f, "")
/// #     }
/// # }
/// ```
///
/// You can use a custom [Scheduler](crate::scheduler::Scheduler) implementation by using
/// the `WolfEngineBuilder::with_custom_scheduler()` method.
///
/// ```
/// # use wolf_engine::{
/// #    ContextBuilder, WolfEngineBuilder, scheduler::FixedUpdateSchedulerBuilder
/// # };
/// #
/// # let context = ContextBuilder::new()
/// #    .build();
/// #
/// # // Using fixed game loop for the example because the actual loop is unimportant.
/// # // Any Scheduler can be provided here and it will work just the same.
/// # let custom_scheduler = FixedUpdateSchedulerBuilder::new()
/// #   .build();
/// #
/// let engine = WolfEngineBuilder::with_custom_scheduler(custom_scheduler)
///     .build(context);
/// ```
pub trait Scheduler: Display {
    /// Update the game state.
    fn update(&mut self, context: &mut Context, state: &mut dyn State); 

    /// Render the game state.
    fn render(&mut self, context: &mut Context, state: &mut dyn State);
}
