//! Provides built-in [Scheduler](crate::Scheduler) implementations.

mod fixed_update_scheduler;
mod simple_render_scheduler;

pub use fixed_update_scheduler::*;
pub use simple_render_scheduler::*;
