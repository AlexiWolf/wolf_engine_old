use crate::contexts::SchedulerContext;
use crate::{Scheduler, Context, State};
use log::trace;
use std::fmt::{Display, Formatter};
use std::time::{Duration, Instant};

/// Represents the number of ticks in a second (tps.)
pub type TickRate = f64;

/// Provides a [Scheduler] with consistent fixed-time-step updates, and variable
/// rendering.
///
/// # Frame-rate Independence
///
/// No matter what frame-rate the game is running at, the gameplay will stay consistent.
/// The loop will always perform the same number of ticks for a given period of game time,
/// and the time-step for each tick will always be the same. This is achieved by adjusting
/// the number of ticks in response to how far the game has fallen behind where it should
/// be.
///
/// How far behind the game is is called `lag`.  The game is ticked forward until the
/// `lag` is less than the time-step, or until the real update time has exceeded the
/// update time limit.
///
/// This results in the following behavior.
///
/// - At 120 tps and 30 fps, the loop runs 4 x 8ms ticks per frame.
/// - At 120 tps and 60 fps, the loop runs 2 x 8ms ticks per frame.
/// - At 120 tps and 120 fps, the loop runs 1 x 8ms ticks per frame.
/// - At 120 tps and 240 fps, the loop runs 1 x 8ms tick every 2 frames.
///
/// # Dealing With Excess Lag
///
/// Sometimes the `lag` will not be cleared all the way to 0.  In other cases, large
/// lag-spikes may cause the game to exceed it's update time limit.  In these cases, the
/// remaining `lag` is carried over to the next update call and more ticks will be run to
/// catch back up.
///
/// A side-effect of this system is that sometimes frames will be rendered in between
/// ticks.  This can result in ugly stuttering.  To mitigate this, the render function
/// can use the remaining lag to interpolate and smooth the rendered frame between the
/// current one and the next one.
///
/// # Examples
///
/// The [FixedUpdateSchedulerBuilder] should be used to build new instances of the
/// scheduler.
///
/// ```
/// # use wolf_engine::schedulers::FixedUpdateSchedulerBuilder;
/// #
/// let mut scheduler = FixedUpdateSchedulerBuilder::new()
///     .build();
/// ```
///
/// The scheduler can then be used by calling `update` and `render` in a loop. A Game's
/// [State], along with the [Context] object are passed in.
///
/// ```
/// # use wolf_engine::{EmptyState, Context, Scheduler, schedulers::FixedUpdateSchedulerBuilder};
/// # let mut scheduler = FixedUpdateSchedulerBuilder::new()
/// #     .build();
/// # let mut context = Context::default();
/// #
/// # let mut state = EmptyState;
/// #
/// loop {
///     scheduler.update(&mut context, &mut state);
///     scheduler.render(&mut context, &mut state);
/// #   break;
/// }
/// ```
///
pub struct FixedUpdateScheduler {
    tps: TickRate,
    max_update_time: Duration,
    update_time: Duration,
    previous_update: Instant,
    lag: Duration,
}

impl FixedUpdateScheduler {
    /// Create a new fixed update scheduler with the default settings.
    pub fn new() -> Self {
        let now = Instant::now();
        let zero = Duration::from_secs(0);
        Self {
            tps: 120.0,
            max_update_time: Duration::from_millis(100),
            update_time: zero,
            previous_update: now,
            lag: zero,
        }
    }

    /// Returns the current target [TickRate] of the scheduler.
    pub fn tps(&self) -> TickRate {
        self.tps
    }

    /// Returns the max update [Duration] allowed by the scheduler.
    pub fn max_update_time(&self) -> Duration {
        self.max_update_time
    }

    /// Returns true if a tick can be run.
    ///
    /// A tick can be run if:
    ///
    /// - The current `lag` is greater than the `time_step` set by the [TickRate].
    /// - The time spent in the current update loop has not exceeded the maximum update
    ///   time.
    pub fn can_run_a_tick(&self) -> bool {
        self.lag_is_greater_than_time_step() && self.has_not_exceeded_max_update_time()
    }

    fn lag_is_greater_than_time_step(&self) -> bool {
        self.lag >= self.time_step()
    }

    fn has_not_exceeded_max_update_time(&self) -> bool {
        self.update_time < self.max_update_time
    }

    fn time_step(&self) -> Duration {
        Duration::from_millis((1000.0 / self.tps).round() as u64)
    }

    fn time_since_last_update(&mut self) -> (Instant, Duration) {
        let current_instant = Instant::now();
        let elapsed_time = current_instant - self.previous_update;
        (current_instant, elapsed_time)
    }

    fn accumulate_lag(&mut self) {
        let (current_instant, elapsed_time) = self.time_since_last_update();
        self.previous_update = current_instant;
        self.lag += elapsed_time;
    }

    fn update_timing(&mut self, tick_run_time: Duration) {
        self.update_time += tick_run_time;
        self.lag -= self.time_step();
    }

    fn run_tick_loop(&mut self, state: &mut dyn State, context: &mut Context) {
        while self.can_run_a_tick() {
            trace!("Running Tick: {}", self);
            self.tick(state, context);
        }
    }

    fn tick(&mut self, state: &mut dyn State, context: &mut Context) {
        let tick_run_time = Self::run_tick_and_track_execution_time(state, context);
        self.update_timing(tick_run_time);
        if let Some(scheduler_context) = context.get_subcontext_mut::<SchedulerContext>() {
            scheduler_context.add_tick();
        }
    }

    fn run_tick_and_track_execution_time(state: &mut dyn State, context: &mut Context) -> Duration {
        let tick_start = Instant::now();
        state.update(context);
        tick_start.elapsed()
    }
}

impl Scheduler for FixedUpdateScheduler {
    fn update(&mut self, context: &mut Context, state: &mut dyn State) {
        self.accumulate_lag();
        self.run_tick_loop(state, context);
        self.update_time = Duration::from_secs(0);
    }

    fn render(&mut self, context: &mut Context, state: &mut dyn State) {
        state.render(context);
        if let Some(scheduler_context) = context.get_subcontext_mut::<SchedulerContext>() {
            scheduler_context.add_frame();
        }
    }
}

impl Display for FixedUpdateScheduler {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Lag: {}ms, Update Time: {}ms / {}ms",
            self.lag.as_millis(),
            self.update_time.as_millis(),
            self.max_update_time.as_millis()
        )
    }
}

impl Default for FixedUpdateScheduler {
    fn default() -> Self {
        Self::new()
    }
}

/// Builds an instance of [FixedUpdateScheduler].
pub struct FixedUpdateSchedulerBuilder {
    scheduler: FixedUpdateScheduler,
}

impl FixedUpdateSchedulerBuilder {
    /// Create a new fixed scheduler builder with the default settings.
    pub fn new() -> Self {
        Self {
            scheduler: FixedUpdateScheduler::default(),
        }
    }

    /// Set the target [TickRate], or ticks per second of the scheduler.
    pub fn with_tps(mut self, tps: TickRate) -> Self {
        self.scheduler.tps = tps;
        self
    }

    /// Set the max update time of the scheduler.
    pub fn with_max_update_time(mut self, max_update_time: Duration) -> Self {
        self.scheduler.max_update_time = max_update_time;
        self
    }

    /// Build an instance of [FixedUpdateScheduler] using the builder.
    pub fn build(self) -> FixedUpdateScheduler {
        self.scheduler
    }
}

impl Default for FixedUpdateSchedulerBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod fixed_update_scheduler_tests {
    use super::*;
    use crate::Context;
    use crate::MockState;
    use std::thread;
    use test_case::test_case;

    #[test_case(800, 0; "with 800 ms of lag")]
    #[test_case(80, 0; "with 80 ms of lag")]
    #[test_case(8, 0; "with 8 ms of lag")]
    #[test_case(8, 99; "with 99 ms of update time")]
    fn should_update(lag: u64, update_time: u64) {
        let (scheduler, _) = test_scheduler(lag, update_time);
        assert!(
            scheduler.can_run_a_tick(),
            "The scheduler should be able to update with {}ms of lag and {}ms of update time.",
            scheduler.lag.as_millis(),
            scheduler.update_time.as_millis()
        );
    }

    #[test_case(7, 0; "with 7 ms of lag")]
    #[test_case(5, 0; "with 5 ms of lag")]
    #[test_case(0, 0; "with 0 ms of lag")]
    #[test_case(8, 101; "with 101 ms of update time")]
    #[test_case(8, 100; "with 100 ms of update time")]
    fn should_not_update(lag: u64, update_time: u64) {
        let (scheduler, _) = test_scheduler(lag, update_time);
        assert!(
            !scheduler.can_run_a_tick(),
            "The scheduler should not be able to update with {}ms of lag and {}ms of update time.",
            scheduler.lag.as_millis(),
            scheduler.update_time.as_millis()
        );
    }

    #[test_case(0, 2 ; "with 0 ms of update time")]
    #[test_case(6, 1 ; "with 6 ms of update time")]
    fn should_stop_ticking_if_max_update_time_is_reached(tick_delay: u64, ticks: usize) {
        let (mut scheduler, mut context) = test_scheduler(16, 0);
        let mut state = MockState::new();
        state.expect_update().times(ticks).returning(move |_| {
            thread::sleep(Duration::from_millis(tick_delay));
            None
        });
        scheduler.max_update_time = Duration::from_millis(5);

        scheduler.update(&mut context, &mut state);
    }

    #[test]
    fn should_call_the_update_function() {
        let (mut scheduler, mut context) = test_scheduler(8, 0);
        let mut state = MockState::new();
        state.expect_update().times(1..).returning(|_| None);

        scheduler.update(&mut context, &mut state);
    }

    #[test]
    fn should_call_the_render_function() {
        let (mut scheduler, mut context) = test_scheduler(8, 0);
        let mut state = MockState::new();
        state.expect_render().times(1).returning(|_| ());

        scheduler.render(&mut context, &mut state);
    }

    /// Testing minimum ticks because this test is not consistent cross platforms when checking
    /// exact values.  Windows and Mac, for example, tend to spend more time than specified sleeping
    /// which results in the number of updates exceeding that exact value.  THIS BEHAVIOR IS
    /// CORRECT, so instead of checking for exact values, a target value is provided and the game
    /// loop must tick AT LEAST that many times.
    #[test_case(120.0, 30, 4  ; "4 times at 120 tps and 30 fps")]
    #[test_case(120.0, 60, 2  ; "2 times at 120 tps and 60 fps")]
    #[test_case(120.0, 120, 1 ; "1 time at 120 tps and 120 fps")]
    fn should_tick_at_least(tick_rate: f64, fps: u64, minimum_ticks: u64) {
        let (mut scheduler, mut context) = test_scheduler(0, 0);
        scheduler.tps = tick_rate;
        let mut state = MockState::new();
        state.expect_update().returning(|_| None);

        thread::sleep(Duration::from_millis(1000 / fps));
        scheduler.update(&mut context, &mut state);

        let scheduler_context = context
            .get_subcontext::<SchedulerContext>()
            .expect("no SchedulerContext");
        assert!(
            scheduler_context.ticks() >= minimum_ticks,
            "The scheduler did not reach the expected number of ticks"
        )
    }

    #[test]
    fn should_count_frames_rendered() {
        let (mut scheduler, mut context) = test_scheduler(0, 0);
        let mut state = MockState::new();
        state.expect_render().times(10).returning(|_| ());

        for _ in 0..10 {
            scheduler.render(&mut context, &mut state);
        }

        let scheduler_context = context
            .get_subcontext::<SchedulerContext>()
            .expect("no SchedulerContext");
        assert_eq!(
            scheduler_context.frames(),
            10,
            "The scheduler should have counted 10 frames.",
        )
    }

    #[test]
    fn should_reset_the_update_time_each_frame() {
        let (mut scheduler, mut context) = test_scheduler(0, 0);
        let mut state = MockState::new();
        state.expect_update().returning(|_| None);

        for _ in 0..5 {
            assert_eq!(
                scheduler.update_time.as_millis(),
                0,
                "The update time was not reset."
            );
            scheduler.lag = Duration::from_millis(8);
            scheduler.update(&mut context, &mut state);
        }
    }

    fn test_scheduler(
        artificial_lag: u64,
        artificial_update_time: u64,
    ) -> (FixedUpdateScheduler, Context) {
        let mut scheduler = FixedUpdateSchedulerBuilder::new().build();
        scheduler.lag = Duration::from_millis(artificial_lag);
        scheduler.update_time = Duration::from_millis(artificial_update_time);
        let context = Context::default();
        (scheduler, context)
    }
}

#[cfg(test)]
mod fixed_update_scheduler_builder_tests {
    use super::*;

    #[test]
    fn should_have_default_values_in_builder() {
        let scheduler = FixedUpdateSchedulerBuilder::new().build();

        assert_eq!(scheduler.tps(), 120.0);
        assert_eq!(scheduler.max_update_time(), Duration::from_millis(100));
    }

    #[test]
    fn should_have_tps_setter() {
        let scheduler = FixedUpdateSchedulerBuilder::new().with_tps(60.0).build();

        assert_eq!(scheduler.tps(), 60.0);
    }

    #[test]
    fn should_have_max_update_time_setter() {
        let scheduler = FixedUpdateSchedulerBuilder::new()
            .with_max_update_time(Duration::from_secs(1))
            .build();

        assert_eq!(scheduler.max_update_time(), Duration::from_secs(1));
    }
}
