use std::fmt::{Display, Formatter};
use std::time::{Duration, Instant};

use wolf_engine_core::*;

use crate::contexts::SchedulerContext;
use crate::schedulers::UpdateScheduler;
use crate::stages::*;
use crate::*;

use log::trace;

/// Represents the number of ticks in a second (tps.)
pub type TickRate = f64;

/// Provides an [UpdateScheduler] with consistent, framerate-independent, fixed time steps.
///
/// Based on [Fix Your Timestep](https://www.gafferongames.com/post/fix_your_timestep/).
///
/// No matter what framerate the game is running at, the game will run a consistent speed.
/// The scheduler will always perform the same number of ticks for a given period of game time,
/// and the timestep for each tick will always be the same. This is achieved by adjusting
/// the number of ticks in response to how much real time has passed between the last update, and
/// the current update.
///
/// The amount of real time between the last update and the current update is called the `lag`.  
/// The game is stepped forward in consistent timesteps until the `lag` is less than the timestep,
/// or the real update time has exceeded the update time limit.  For example, assuming a tickrate
/// of 120 ticks / sec, you can expect the following behavior:
///
/// - 4 x 8ms ticks per frame at 30 fps.
/// - 2 x 8ms ticks per frame at 60 fps.
/// - 1 x 8ms ticks per frame at 120 fps.
/// - 1 x 8ms tick every 2 frames at 240 fps.
///
/// In practice, the framerate, tick execution speed, and number of ticks ran is unlikely to be
/// exact.  Sometimes the `lag` will not be cleared all the way to 0, and in other cases, large
/// lag spikes may cause the game to exceed it's update time limit.  If there is remaining `lag`,
/// the `lag` is carried over to the next update cycle and more ticks will run to catch up.  For
/// large lag spikes, the game may temporarily slow down, but it should catch back up within a few
/// frames, and the number of ticks ran will stay consistent.
///
/// # Dealing With Choppy Gameplay From Residual Lag
///
/// If the lag is not cleared all the way to 0, the frame will be rendering the game between two
/// updates.  This can happen when there is no clean way to divide the frame rate by the tick rate.
/// This can lead to visibly choppy, and "laggy" feeling gameplay, especially at lower tickrates.
/// To solve this problem, the renderer can interpolate between the previous state, and the current
/// state to smooth the motion.
///
/// # Examples
///
/// The [FixedUpdateSchedulerBuilder] should be used to build new instances of the scheduler.
///
/// ```
/// # use wolf_engine_framework::schedulers::FixedUpdateScheduler;
/// #
/// let mut scheduler = FixedUpdateScheduler::builder()
///     // Changes settings.
///     .build();
/// ```
///
/// To use the [Default] settings.
///
/// ```
/// # use wolf_engine_framework::schedulers::FixedUpdateScheduler;
/// #
/// let mut scheduler = FixedUpdateScheduler::default();
/// ```
///
/// To run the scheduler, use [UpdateScheduler::update()].
#[derive(Debug)]
pub struct FixedUpdateScheduler {
    tps: TickRate,
    time_step: Duration,
    max_update_time: Duration,
    update_time: Duration,
    previous_update: Instant,
    lag: Duration,
}

impl UpdateScheduler for FixedUpdateScheduler {
    fn update(
        &mut self,
        context: &mut Context,
        state: &mut dyn State,
        stage_callbacks: &mut StageCallbacks,
    ) {
        self.accumulate_lag();
        self.run_tick_loop(state, context, stage_callbacks);
        self.update_time = Duration::from_secs(0);
    }
}

impl FixedUpdateScheduler {
    pub fn builder() -> FixedUpdateSchedulerBuilder {
        FixedUpdateSchedulerBuilder::new()
    }

    fn time_step(tick_rate: TickRate) -> Duration {
        Duration::from_millis((1000.0 / tick_rate).round() as u64)
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
        self.lag >= self.time_step
    }

    fn has_not_exceeded_max_update_time(&self) -> bool {
        self.update_time < self.max_update_time
    }

    fn accumulate_lag(&mut self) {
        let (current_instant, elapsed_time) = self.time_since_last_update();
        self.previous_update = current_instant;
        self.lag += elapsed_time;
    }

    fn time_since_last_update(&mut self) -> (Instant, Duration) {
        let current_instant = Instant::now();
        let elapsed_time = current_instant - self.previous_update;
        (current_instant, elapsed_time)
    }

    fn run_tick_loop(
        &mut self,
        state: &mut dyn State,
        context: &mut Context,
        stage_callbacks: &mut StageCallbacks,
    ) {
        while self.can_run_a_tick() {
            trace!("Running Tick: {}", self);
            stage_callbacks.run(StageType::PreUpdate, context);
            stage_callbacks.run(StageType::Update, context);
            self.run_tick(state, context);
            stage_callbacks.run(StageType::PostUpdate, context);
        }
    }

    fn run_tick(&mut self, state: &mut dyn State, context: &mut Context) {
        let tick_run_time = Self::run_tick_and_track_execution_time(state, context);
        self.update_timing(tick_run_time);
        if let Some(mut scheduler_context) = context.borrow_mut::<SchedulerContext>() {
            scheduler_context.add_tick();
        }
    }

    fn run_tick_and_track_execution_time(state: &mut dyn State, context: &mut Context) -> Duration {
        let tick_start = Instant::now();
        state.update(context);
        tick_start.elapsed()
    }

    fn update_timing(&mut self, tick_run_time: Duration) {
        self.update_time += tick_run_time;
        self.lag -= self.time_step;
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
        let now = Instant::now();
        let zero = Duration::from_secs(0);
        let tps = 120.0;
        Self {
            tps,
            time_step: Self::time_step(tps),
            max_update_time: Duration::from_millis(100),
            update_time: zero,
            previous_update: now,
            lag: zero,
        }
    }
}

/// Builds an instance of [FixedUpdateScheduler].
#[derive(Debug)]
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
        self.scheduler.time_step = FixedUpdateScheduler::time_step(tps);
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
    use crate::MockState;
    use std::thread;
    use test_case::test_case;
    use wolf_engine_core::Context;

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

        scheduler.update(&mut context, &mut state, &mut StageCallbacks::new());
    }

    #[test]
    fn should_call_the_update_function() {
        let (mut scheduler, mut context) = test_scheduler(8, 0);
        let mut state = MockState::new();
        state.expect_update().times(1..).returning(|_| None);

        scheduler.update(&mut context, &mut state, &mut StageCallbacks::new());
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
        scheduler.update(&mut context, &mut state, &mut StageCallbacks::new());

        let scheduler_context = context
            .borrow::<SchedulerContext>()
            .expect("no SchedulerContext");
        assert!(
            scheduler_context.ticks() >= minimum_ticks,
            "The scheduler did not reach the expected number of ticks"
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
            scheduler.update(&mut context, &mut state, &mut StageCallbacks::new());
        }
    }

    #[test]
    fn should_run_update_stages() {
        let (scheduler, _) = test_scheduler(8, 0);
        scheduler_integration_tests::should_run_update_stages(scheduler);
    }

    fn test_scheduler(
        artificial_lag: u64,
        artificial_update_time: u64,
    ) -> (FixedUpdateScheduler, Context) {
        let mut scheduler = FixedUpdateSchedulerBuilder::new().build();
        scheduler.lag = Duration::from_millis(artificial_lag);
        scheduler.update_time = Duration::from_millis(artificial_update_time);
        let mut context = Context::default();
        context.add(SchedulerContext::new()).unwrap();
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
    fn should_set_tick_rate_and_update_time_step() {
        let scheduler = FixedUpdateSchedulerBuilder::new().with_tps(60.0).build();

        assert_eq!(scheduler.tps(), 60.0);
        assert_eq!(scheduler.time_step, Duration::from_millis(17));
    }

    #[test]
    fn should_have_max_update_time_setter() {
        let scheduler = FixedUpdateSchedulerBuilder::new()
            .with_max_update_time(Duration::from_secs(1))
            .build();

        assert_eq!(scheduler.max_update_time(), Duration::from_secs(1));
    }
}