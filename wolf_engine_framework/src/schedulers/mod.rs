//! Provides flexible timing controls for the [Engine].
//!
//! A scheduler is responsible for determining if and when the [Engine] should run various
//! operations.  This includes when to run [State] updates, render frames, and run various engine
//! [stages](crate::stages).  Wolf Engine has two kinds of scheduler, [UpdateScheduler] to control
//! game logic, and [RenderScheduler] to control rendering.  Refer to the respective trait's
//! documentation for
//! specific details.
//!
//! The scheduler traits make it easy to change the behavior of the [Engine] to suit your game's
//! needs. Different implementations may provide different behavior and additional features, so you
//! should refer to implementation docs for specific details.
//!
//! By default, the [Engine] will use a [FixedUpdateScheduler](crate::schedulers::FixedUpdateScheduler)
//! and a [SimpleRenderScheduler](crate::schedulers::SimpleRenderScheduler).
//!
//! # Examples
//!
//! Alternative schedulers can be provided to the [Engine] at startup using
//! [EngineBuilder::with_update_scheduler()] and [EngineBuilder::with_render_scheduler()].
//!
//! ```
//! # use wolf_engine_framework::*;
//! # use wolf_engine_framework::schedulers::*;
//! #
//! # let update_scheduler = FixedUpdateScheduler::default();
//! # let render_scheduler = SimpleRenderScheduler;
//! #
//! let engine = EngineBuilder::new()
//!     .with_update_scheduler(Box::from(update_scheduler))
//!     .with_render_scheduler(Box::from(render_scheduler))
//!     .build();
//! ```
//!
//! Custom schedulers can be created by implementing a scheduler trait.
//!
//! ```
//! # use wolf_engine_core::*;
//! # use wolf_engine_framework::*;
//! # use wolf_engine_framework::schedulers::*;
//! # use wolf_engine_framework::stages::*;
//! #
//! #[derive(Debug)]
//! pub struct MySimpleUpdateScheduler;
//!
//! impl UpdateScheduler for MySimpleUpdateScheduler {
//!     fn update(
//!         &mut self,
//!         context: &mut Context,
//!         state: &mut dyn State,
//!         stage_callbacks: &mut StageCallbacks
//!     ) {
//!         stage_callbacks.run(StageType::PreUpdate, context);
//!         stage_callbacks.run(StageType::Update, context);
//!         state.update(context);
//!         stage_callbacks.run(StageType::PostUpdate, context);
//!     }
//! }
//!
//! #[derive(Debug)]
//! pub struct MySimpleRenderScheduler;
//!
//! impl RenderScheduler for MySimpleRenderScheduler {
//!     fn render(
//!         &mut self,
//!         context: &mut Context,
//!         state: &mut dyn State,
//!         stage_callbacks: &mut StageCallbacks
//!     ) {
//!         stage_callbacks.run(StageType::PreRender, context);
//!         stage_callbacks.run(StageType::Render, context);
//!         state.render(context);
//!         stage_callbacks.run(StageType::PostRender, context);
//!     }
//! }
//! ```

mod fixed_update_scheduler;
mod simple_render_scheduler;

use std::fmt::Debug;

use wolf_engine_core::*;

use crate::stages::StageCallbacks;
use crate::*;

pub use fixed_update_scheduler::*;
pub use simple_render_scheduler::*;

#[cfg(test)]
use mockall::automock;

/// Controls how and when the the game / engine state is updated.
#[cfg_attr(test, automock)]
pub trait UpdateScheduler: Debug {
    /// Update the game state.
    fn update(
        &mut self,
        context: &mut Context,
        state: &mut dyn State,
        stage_callbacks: &mut StageCallbacks,
    );
}

/// Controls how and when a frame should be rendered.
#[cfg_attr(test, automock)]
pub trait RenderScheduler: Debug {
    /// Render the current frame.
    fn render(
        &mut self,
        context: &mut Context,
        state: &mut dyn State,
        stage_callbacks: &mut StageCallbacks,
    );
}
