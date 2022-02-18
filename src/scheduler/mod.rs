//! Provides flexible update and frame timing controls for [WolfEngine](crate::WolfEngine).

mod fixed_update_scheduler;

pub use fixed_update_scheduler::*;

use crate::{Context, State};

/// Represents the number of ticks the engine has run.
pub type Ticks = u64;

/// Represents the number of frames the engine has rendered.
pub type Frames = u64;

/// Controls how the game is run.
///
/// The scheduler is responsible for determining if or when the game should be updated 
/// or rendered.  Schedulers are useful for creating predictable update timing, preventing 
/// busy loops, and implementing frame limiters, among other things.
///
/// Wolf Engine's default scheduler is the
/// [FixedUpdateScheduler](fixed_update_scheduler::FixedUpdateScheduler). 
///
/// Different schedulers may operate differently, so you should refer to implementation
/// documentation for specific details.
///
/// ## Updating
///
/// At it's most basic, the `update` function just calls the game's update function and 
/// returns the result from it.  Depending on the implementation, different update 
/// strategies may be used and / or additional timing controls may be added.
///
/// Ticks vs Updates:
///
/// - An *update* refers to a single call to the scheduler's `update` method.
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
/// # Creating a Custom Scheduler
///
/// Wolf Engine fully supports using a custom scheduler.  Simply implement this trait.
///
/// ```
/// use wolf_engine::{State, Context, scheduler::Scheduler};
///
/// pub struct MyScheduler;
///
/// impl Scheduler for MyScheduler {
///
///     fn update(&mut self, context: &mut Context, state: &mut dyn State) {
///         // Add timing control logic here.
///         state.update(context);
///     }
///
///     fn render(&mut self, context: &mut Context, state: &mut dyn State) {
///         // Add timing control logic here.
///         state.render(context);
///     }
/// }
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
pub trait Scheduler {
    /// Update the game state.
    fn update(&mut self, context: &mut Context, state: &mut dyn State); 

    /// Render the game state.
    fn render(&mut self, context: &mut Context, state: &mut dyn State);
}
