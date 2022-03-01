use std::sync::{Arc, Mutex};

use crate::scheduler::{Frames, Ticks};

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
/// # let scheduler_context = SchedulerContext::new();
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
    ticks: Arc<Mutex<Ticks>>,
    frames: Arc<Mutex<Frames>>,
}

impl SchedulerContext {
    /// Creates a new instance with all values starting at 0. 
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Increment the number of ticks by 1.
    pub fn add_tick(&self) {
        *self.ticks.lock().unwrap() += 1;
    }
    
    /// Access the current number of counted ticks.
    pub fn ticks(&self) -> Ticks {
        *self.ticks.lock().unwrap()
    }

    /// Increment the number of frames by 1.
    pub fn add_frame(&self) {
        *self.frames.lock().unwrap() += 1;
    }

    /// Access the current number of counted frames.
    pub fn frames(&self) -> Frames {
        *self.frames.lock().unwrap()
    }
}

impl Default for SchedulerContext {
    fn default() -> Self {
        Self {
            ticks: Arc::from(Mutex::from(0)),
            frames: Arc::from(Mutex::from(0)),
        }
    }
}
