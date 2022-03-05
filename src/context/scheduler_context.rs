use crate::scheduler::{Frames, Ticks};

use super::Subcontext;

/// Provides access to information and controls for the
/// [Scheduler](crate::scheduler::Scheduler).
///
/// # Examples
///
/// The SchedulerContext can be created directly using the new method.
///
/// ```
/// # use wolf_engine::context::SchedulerContext;
/// #
/// let scheduler_context = SchedulerContext::new();
/// ```
///
/// Once created, the SchedulerContext exposes information about the
/// [Scheduler](crate::scheduler::Scheduler).
///
/// ```
/// # use wolf_engine::context::SchedulerContext;
/// #
/// # let scheduler_context = SchedulerContext::new();
/// #
/// scheduler_context.ticks();
/// scheduler_context.frames();
/// ```
///
/// Tick and frame information can be added to the context.  
///
/// ```
/// # use wolf_engine::context::SchedulerContext;
/// #
/// # let mut scheduler_context = SchedulerContext::new();
/// #
/// # assert_eq!(scheduler_context.ticks(), 0, "There should be 0 ticks before add_tick is called");
/// # assert_eq!(scheduler_context.frames(), 0, "There should be 0 frames before add_tick is called");
/// #
/// scheduler_context.add_tick();
/// scheduler_context.add_frame();
/// #
/// # scheduler_context.ticks();
/// # scheduler_context.frames();
/// #
/// # assert_eq!(scheduler_context.ticks(), 1, "1 tick should have been added");
/// # assert_eq!(scheduler_context.frames(), 1, "1 frame should have been added");
/// ```
pub struct SchedulerContext {
    ticks: Ticks,
    frames: Frames,
}

impl SchedulerContext {
    /// Creates a new instance with all values starting at 0.
    pub fn new() -> Self {
        Self::default()
    }

    /// Increment the number of ticks by 1.
    pub fn add_tick(&mut self) {
        self.ticks += 1;
    }

    /// Access the current number of counted ticks.
    pub fn ticks(&self) -> Ticks {
        self.ticks 
    }

    /// Increment the number of frames by 1.
    pub fn add_frame(&mut self) {
        self.frames += 1;
    }

    /// Access the current number of counted frames.
    pub fn frames(&self) -> Frames {
        self.frames
    }
}

impl Default for SchedulerContext {
    fn default() -> Self {
        Self {
            ticks: 0,
            frames: 0,
        }
    }
}

impl Subcontext for SchedulerContext {}

