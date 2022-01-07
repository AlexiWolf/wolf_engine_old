use crate::{
    game_loop::{GameLoop, LoopResult},
    Context, State,
};
use log::trace;
use std::fmt::{Display, Formatter};
use std::time::{Duration, Instant};

/// Represents the number of ticks in a second (tps.)
pub type TickRate = f64;

/// Provides a [GameLoop] with consistent fixed-time-step updates, and variable rendering.
///
/// # Frame-rate Independence
///
/// No matter what frame-rate the game is running at, the gameplay will stay consistent.  The loop
/// will always perform the same number of ticks for a given period of game time, and the time-step
/// for each tick will always be the same. This is achieved by adjusting the number of ticks in
/// response to how far the game has fallen behind where it should be.
///
/// How far behind the game is is called `lag`.  The game is ticked forward until the `lag` is
/// less than the time-step, or until the real update time has exceeded the update time limit.
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
/// Sometimes the `lag` will not be cleared all the way to 0.  In other cases, large lag-spikes may
/// cause the game to exceed it's update time limit.  In these cases, the remaining `lag` carried
/// over to the next update call and more ticks will be run to catch back up.
///
/// A side-effect of this system is that sometimes frames will be rendered in between ticks.  This
/// can result in ugly stuttering.  To mitigate this, the render function can use the remaining lag
/// to interpolate and smooth the rendered frame between the current one and the next one.
///
/// # Examples
///
/// The [FixedUpdateGameLoopBuilder] should be used to build new instances of the loop.
///
/// ```
/// # use wolf_engine::game_loop::FixedUpdateGameLoopBuilder;
///
/// let mut game_loop = FixedUpdateGameLoopBuilder::new()
///     .build();
/// ```
///
/// The game loop can then be used by calling `update` and `render` in a loop.  The game's
/// update and render functions, along with the [Context] object are passed in.
///
/// ```
/// # use wolf_engine::{ContextBuilder, game_loop::{GameLoop, FixedUpdateGameLoopBuilder}};
/// # let mut game_loop = FixedUpdateGameLoopBuilder::new()
/// #     .build();
/// # let mut context = ContextBuilder::new().build();
/// #
/// loop {
///     game_loop.update(&mut context, |context| {
///         // Update logic goes here.
///     });
///     game_loop.render(&mut context, |context| {
///         // Render logic goes here.
///     });
/// #   break;
/// }
/// ```
///
pub struct FixedUpdateGameLoop {
    tps: TickRate,
    max_update_time: Duration,
    update_time: Duration,
    previous_update: Instant,
    lag: Duration,
}

impl FixedUpdateGameLoop {
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

    pub fn tps(&self) -> TickRate {
        self.tps
    }

    pub fn max_update_time(&self) -> Duration {
        self.max_update_time
    }

    pub fn can_update(&self) -> bool {
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
        while self.can_update() {
            trace!("Running Tick: {}", self);
            self.tick(state, context);
        }
    }

    fn tick(&mut self, state: &mut dyn State, context: &mut Context) {
        let tick_run_time = Self::run_tick_and_track_execution_time(state, context);
        self.update_timing(tick_run_time);
        context.game_loop.add_tick();
    }
    
    fn run_tick_and_track_execution_time(
        state: &mut dyn State,
        context: &mut Context,
    ) -> Duration {
        let tick_start = Instant::now();
        state.update(context);
        tick_start.elapsed()
    }
}

impl GameLoop for FixedUpdateGameLoop {
    fn update(&mut self, context: &mut Context, state: &mut dyn State) -> LoopResult {
        self.accumulate_lag();
        self.run_tick_loop(state, context);
        self.update_time = Duration::from_secs(0);
    }

    fn render(&mut self, context: &mut Context, state: &mut dyn State) -> LoopResult {
        state.render(context);
        context.game_loop.add_frame();
    }
}

impl Display for FixedUpdateGameLoop {
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

impl Default for FixedUpdateGameLoop {
    fn default() -> Self {
        Self::new()
    }
}

/// Builds an instance of [FixedUpdateGameLoop].
pub struct FixedUpdateGameLoopBuilder {
    game_loop: FixedUpdateGameLoop,
}

impl FixedUpdateGameLoopBuilder {
    pub fn new() -> Self {
        Self {
            game_loop: FixedUpdateGameLoop::default(),
        }
    }

    pub fn with_tps(mut self, tps: TickRate) -> Self {
        self.game_loop.tps = tps;
        self
    }

    pub fn with_max_update_time(mut self, max_update_time: Duration) -> Self {
        self.game_loop.max_update_time = max_update_time;
        self
    }

    pub fn build(self) -> FixedUpdateGameLoop {
        self.game_loop
    }
}

impl Default for FixedUpdateGameLoopBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod fixed_update_game_loop_tests {
    use super::*;
    use crate::MockState;
    use crate::{game_loop::Ticks, Context, ContextBuilder};
    use std::sync::{Arc, Mutex};
    use std::thread;
    use mockall::predicate;
    use test_case::test_case;

    #[test_case(800, 0; "with 800 ms of lag")]
    #[test_case(80, 0; "with 80 ms of lag")]
    #[test_case(8, 0; "with 8 ms of lag")]
    #[test_case(8, 99; "with 99 ms of update time")]
    fn should_update(lag: u64, update_time: u64) {
        let (game_loop, _) = test_game_loop(lag, update_time);
        assert!(
            game_loop.can_update(),
            "The game loop should be able to update with {}ms of lag and {}ms of update time.",
            game_loop.lag.as_millis(),
            game_loop.update_time.as_millis()
        );
    }

    #[test_case(7, 0; "with 7 ms of lag")]
    #[test_case(5, 0; "with 5 ms of lag")]
    #[test_case(0, 0; "with 0 ms of lag")]
    #[test_case(8, 101; "with 101 ms of update time")]
    #[test_case(8, 100; "with 100 ms of update time")]
    fn should_not_update(lag: u64, update_time: u64) {
        let (game_loop, _) = test_game_loop(lag, update_time);
        assert!(
            !game_loop.can_update(),
            "The game loop should not be able to update with {}ms of lag and {}ms of update time.",
            game_loop.lag.as_millis(),
            game_loop.update_time.as_millis()
        );
    }

    #[test_case(0, 2 ; "with 0 ms of update time")]
    #[test_case(6, 1 ; "with 6 ms of update time")]
    fn should_stop_ticking_if_max_update_time_is_reached(tick_delay: u64, ticks: usize) {
        let (mut game_loop, mut context) = test_game_loop(16, 0);
        let mut state = MockState::new();
        state.expect_update()
            .times(ticks)
            .returning(move |_| { 
                thread::sleep(Duration::from_millis(tick_delay));
                None
            });
        game_loop.max_update_time = Duration::from_millis(5);

        game_loop.update(&mut context, &mut state);
    }

    #[test]
    fn should_call_the_update_function() {
        let (mut game_loop, mut context) = test_game_loop(8, 0);
        let mut state = MockState::new();
        state.expect_update()
            .times(1..)
            .returning(|_| None);
        
        game_loop.update(&mut context, &mut state);
    }

    #[test]
    fn should_call_the_render_function() {
        let (mut game_loop, mut context) = test_game_loop(8, 0);
        let mut state = MockState::new();
        state.expect_render()
            .times(1)
            .returning(|_| ());

        game_loop.render(&mut context, &mut state);
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
        let (mut game_loop, mut context) = test_game_loop(0, 0);
        game_loop.tps = tick_rate;
        let mut state = MockState::new();
        state.expect_update()
            .returning(|_| None);

        thread::sleep(Duration::from_millis(1000 / fps));
        game_loop.update(&mut context, &mut state);

        assert!(
            context.game_loop.ticks() >= minimum_ticks,
            "The game loop did not reach the expected number of ticks"
        )
    }

    #[test]
    fn should_count_frames_rendered() {
        let (mut game_loop, mut context) = test_game_loop(0, 0);
        let mut state = MockState::new();
        state.expect_render()
            .times(10)
            .returning(|_| ());

        for _ in 0..10 {
            game_loop.render(&mut context, &mut state);
        }

        assert_eq!(
            context.game_loop.frames(),
            10,
            "The game loop should have counted 10 frames.",
        )
    }

    #[test]
    fn should_reset_the_update_time_each_frame() {
        let (mut game_loop, mut context) = test_game_loop(0, 0);
        let mut state = MockState::new();
        state.expect_update()
            .returning(|_| None);

        for _ in 0..5 {
            assert_eq!(
                game_loop.update_time.as_millis(),
                0,
                "The update time was not reset."
            );
            game_loop.lag = Duration::from_millis(8);
            game_loop.update(&mut context, &mut state);
        }
    }

    fn test_game_loop(
        artificial_lag: u64,
        artificial_update_time: u64,
    ) -> (FixedUpdateGameLoop, Context) {
        let mut game_loop = FixedUpdateGameLoopBuilder::new().build();
        game_loop.lag = Duration::from_millis(artificial_lag);
        game_loop.update_time = Duration::from_millis(artificial_update_time);
        let context = ContextBuilder::new().build();
        (game_loop, context)
    }
}

#[cfg(test)]
mod fixed_update_game_loop_builder_tests {
    use super::*;

    #[test]
    fn should_have_default_values_in_builder() {
        let game_loop = FixedUpdateGameLoopBuilder::new().build();

        assert_eq!(game_loop.tps(), 120.0);
        assert_eq!(game_loop.max_update_time(), Duration::from_millis(100));
    }

    #[test]
    fn should_have_tps_setter() {
        let game_loop = FixedUpdateGameLoopBuilder::new().with_tps(60.0).build();

        assert_eq!(game_loop.tps(), 60.0);
    }

    #[test]
    fn should_have_max_update_time_setter() {
        let game_loop = FixedUpdateGameLoopBuilder::new()
            .with_max_update_time(Duration::from_secs(1))
            .build();

        assert_eq!(game_loop.max_update_time(), Duration::from_secs(1));
    }
}
