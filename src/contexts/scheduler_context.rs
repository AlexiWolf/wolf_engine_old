use crate::{Frames, Subcontext, Ticks};

/// Provides a way for the active [Scheduler](crate::Scheduler) to report basic data.  
///
/// The scheduler context allows the [Scheduler](crate::Scheduler) to update information
/// such as number of [Ticks] and [Frames] that have been run.  This information can
/// also be accessed by the rest of the engine.
///
/// # Examples
///
/// The context can be created directly using the new method.
///
/// ```
/// # use wolf_engine::contexts::SchedulerContext;
/// #
/// let scheduler_context = SchedulerContext::new();
/// ```
///
/// In most cases, you don't need to create an instance yourself.  Instead, you will
/// get one from the [Context](crate::Context) object.
///
/// ```
/// # use wolf_engine::*;
/// # use wolf_engine::contexts::*;
/// #
/// # let scheduler_context = SchedulerContext::new();
/// # let mut context = Context::empty();
/// # context.add(scheduler_context);
/// #
/// let scheduler_context = context.borrow::<SchedulerContext>()
///     .expect("no scheduler context");
/// ```
///
/// From there, you can read information about the [Scheduler](crate::Scheduler).
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
/// The expectation is that only the active [Scheduler](crate::Scheduler) will be updating
/// the context, so **you should avoid these functions unless you are implementing a
/// custom [Scheduler](crate::Scheduler).**  Updating the context information requires
/// a mutable reference, so the simplest way to avoid causing trouble is to only access
/// this context immutably as shown in the above examples.
///
/// While it is technically safe to use these functions elsewhere, as in it won't result
/// in unsafety or UB , it may cause other parts of the engine or game to behave
/// incorrectly.
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
    ticks: Ticks,
    frames: Frames,
}

impl SchedulerContext {
    /// Creates a new instance with all values starting at 0.
    pub fn new() -> Self {
        Self::default()
    }

    /// Increment the number of ticks by 1.
    ///
    /// **Note:** This is not intended to be used unless you're implementing a custom
    /// [Scheduler](crate::Scheduler)
    pub fn add_tick(&mut self) {
        self.ticks += 1;
    }

    /// Access the current number of counted ticks.
    pub fn ticks(&self) -> Ticks {
        self.ticks
    }

    /// Increment the number of frames by 1.
    ///
    /// **Note:** This is not intended to be used unless you're implementing a custom
    /// [Scheduler](crate::Scheduler)
    pub fn add_frame(&mut self) {
        self.frames += 1;
    }

    /// Access the current number of counted frames.
    pub fn frames(&self) -> Frames {
        self.frames
    }
}

impl Subcontext for SchedulerContext {}
