use crate::Subcontext;

/// Provides a way for a [scheduler](crate::schedulers) to report basic data.  
///
/// The scheduler context allows a[scheduler](crate::schedulers) to update information
/// such as number of ticks and frames that have been run.
///
/// # Examples
///
/// Accessing the scheduler context.
///
/// ```
/// # use wolf_engine::*;
/// # use wolf_engine::contexts::*;
/// #
/// # let scheduler_context = SchedulerContext::new();
/// # let mut context = Context::new();
/// # context.add(scheduler_context);
/// #
/// let scheduler_context = context.borrow::<SchedulerContext>()
///     .expect("no scheduler context");
/// ```
///
/// Getting tick / frame counts.
///
/// ```
/// # use wolf_engine::contexts::SchedulerContext;
/// #
/// # let scheduler_context = SchedulerContext::new();
/// #
/// scheduler_context.ticks();
/// scheduler_context.frames();
/// ```
///
/// ## Updating Stored Information
///
/// The expectation is that only [schedulers](crate::schedulers) will be updating the context, so
/// **you should avoid calling these functions unless you are implementing a custom
/// [scheduler](crate::schedulers),** otherwise the engine or the game may misbehave.
///
/// ```
/// # use wolf_engine::contexts::SchedulerContext;
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
#[derive(Default)]
pub struct SchedulerContext {
    ticks: u64,
    frames: u64,
}

impl SchedulerContext {
    /// Creates a new instance with all values starting at 0.
    pub fn new() -> Self {
        Self::default()
    }

    /// Increment the number of ticks by 1.
    ///
    /// **Note:** This is not intended to be used unless you're implementing a custom
    /// [scheduler](crate::schedulers).
    pub fn add_tick(&mut self) {
        self.ticks += 1;
    }

    /// Access the current number of counted ticks.
    pub fn ticks(&self) -> u64 {
        self.ticks
    }

    /// Increment the number of frames by 1.
    ///
    /// **Note:** This is not intended to be used unless you're implementing a custom
    /// [scheduler](crate::schedulers).
    pub fn add_frame(&mut self) {
        self.frames += 1;
    }

    /// Access the current number of counted frames.
    pub fn frames(&self) -> u64 {
        self.frames
    }
}

impl Subcontext for SchedulerContext {}
