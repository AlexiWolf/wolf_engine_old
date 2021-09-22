use crate::core::{Context, GameLoop, LoopResult};
use std::time::{Duration, Instant};

pub type TicksPerSecond = f64;
pub type Ticks = u64;
pub type Frames = u64;

pub struct FixedUpdateGameLoop {
    tps: TicksPerSecond,
    max_update_time: Duration,
    previous_update: Instant,
    lag: Duration,
    ticks: Ticks,
    frames: Frames,
}

impl FixedUpdateGameLoop {
    pub fn new() -> Self {
        let now = Instant::now();
        let zero = Duration::from_secs(0);
        Self {
            tps: 120.0,
            max_update_time: Duration::from_millis(100),
            previous_update: now,
            lag: zero,
            ticks: 0,
            frames: 0,
        }
    }

    pub fn tps(&self) -> TicksPerSecond {
        self.tps
    }

    pub fn max_update_time(&self) -> Duration {
        self.max_update_time
    }

    pub fn ticks(&self) -> Ticks {
        self.ticks
    }

    pub fn frames(&self) -> Frames {
        self.frames
    }

    pub fn can_update(&self) -> bool {
        self.lag >= self.time_step()
    }
}

impl GameLoop for FixedUpdateGameLoop {
    fn update<F>(&mut self, context: &mut Context, mut update_function: F) -> LoopResult
    where
        F: FnMut(&mut Context),
    {
        self.accumulate_lag();
        while self.can_update() {
            update_function(context);
            self.lag -= self.time_step();
            self.ticks += 1;
        }
    }

    fn render<F>(&mut self, context: &mut Context, mut render_function: F) -> LoopResult
    where
        F: FnMut(&mut Context),
    {
        render_function(context);
    }
}

impl FixedUpdateGameLoop {
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
}

pub struct FixedUpdateGameLoopBuilder {
    game_loop: FixedUpdateGameLoop,
}

impl FixedUpdateGameLoopBuilder {
    pub fn new() -> Self {
        Self {
            game_loop: FixedUpdateGameLoop::new(),
        }
    }

    pub fn with_tps(mut self, tps: TicksPerSecond) -> Self {
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

#[cfg(test)]
mod fixed_update_game_loop_tests {
    use super::*;
    use crate::core::Context;
    use std::sync::{Arc, Mutex};
    use std::thread;
    use test_case::test_case;

    #[test_case(800; "with 800 ms of lag")]
    #[test_case(80; "with 80 ms of lag")]
    #[test_case(8; "with 8 ms of lag")]
    fn should_update(lag: u64) {
        let game_loop = lag_test_game_loop(lag);
        assert!(
            game_loop.can_update(),
            "The game loop should be able to update with {}ms of lag.",
            game_loop.lag.as_millis()
        );
    }

    #[test_case(7; "with 7 ms of lag")]
    #[test_case(5; "with 5 ms of lag")]
    #[test_case(0; "with 0 ms of lag")]
    fn should_not_update(lag: u64) {
        let game_loop = lag_test_game_loop(lag);
        assert!(
            !game_loop.can_update(),
            "The game loop should not be able to update with {}ms of lag.",
            game_loop.lag.as_millis()
        );
    }

    #[test]
    fn should_call_the_update_function() {
        let has_called_update_function = Arc::from(Mutex::from(false));
        let mut context = Context;
        let mut game_loop = lag_test_game_loop(8);

        game_loop.update(&mut context, |_| {
            let mut has_called_update_function = has_called_update_function.lock().unwrap();
            *has_called_update_function = true;
        });

        let has_called_update_function = has_called_update_function.lock().unwrap();
        assert!(
            *has_called_update_function,
            "The update function was not called."
        );
    }

    #[test]
    fn should_call_the_render_function() {
        let has_called_render_function = Arc::from(Mutex::from(false));
        let mut context = Context;
        let mut game_loop = lag_test_game_loop(8);

        game_loop.render(&mut context, |_| {
            let mut has_called_render_function = has_called_render_function.lock().unwrap();
            *has_called_render_function = true;
        });

        let has_called_render_function = has_called_render_function.lock().unwrap();
        assert!(
            *has_called_render_function,
            "The render function was not called."
        );
    }

    fn lag_test_game_loop(lag: u64) -> FixedUpdateGameLoop {
        let mut game_loop = FixedUpdateGameLoopBuilder::new().build();
        game_loop.lag = Duration::from_millis(lag);
        game_loop
    }

    #[test_case(120.0, 30 => 4  ; "4 times at 120 tps and 30 fps")]
    #[test_case(120.0, 60 => 2  ; "2 times at 120 tps and 60 fps")]
    #[test_case(120.0, 120 => 1 ; "1 time at 120 tps and 120 fps")]
    fn should_tick(tick_rate: f64, fps: u64) -> u64 {
        let mut context = Context;
        let mut game_loop = FixedUpdateGameLoopBuilder::new()
            .with_tps(tick_rate)
            .build();

        thread::sleep(Duration::from_millis(1000 / fps));
        game_loop.update(&mut context, |_| {});

        game_loop.ticks()
    }

    #[test]
    fn should_count_frames_rendered() {
        let mut context = Context;
        let mut game_loop = FixedUpdateGameLoopBuilder::new().build();

        for _ in 0..10 {
            game_loop.render(&mut context, |_|{})
        }

        assert_eq!(
            game_loop.frames(),
            10,
            "The game loop should have counted 10 frames.",
        )
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
