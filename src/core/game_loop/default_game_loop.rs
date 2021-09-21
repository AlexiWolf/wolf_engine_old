use crate::core::{Context, GameLoop, LoopResult};
use std::time::{Duration, Instant};

pub type TicksPerSecond = f64;

pub struct DefaultGameLoop {
    tps: TicksPerSecond,
    max_update_time: Duration,
    last_update_instant: Instant,
    current_update_instant: Instant,
    lag: Duration,
}

impl DefaultGameLoop {
    pub fn new() -> Self {
        let now = Instant::now();
        let zero = Duration::from_secs(0);
        Self {
            tps: 120.0,
            max_update_time: Duration::from_millis(100),
            last_update_instant: now,
            current_update_instant: now,
            lag: zero,
        }
    }

    pub fn tps(&self) -> TicksPerSecond {
        self.tps
    }

    pub fn max_update_time(&self) -> Duration {
        self.max_update_time
    }

    pub fn can_update(&self) -> bool {
        self.lag >= self.time_step()
    }

    fn time_step(&self) -> Duration {
        Duration::from_millis((1000.0 / self.tps).round() as u64)
    }
}

impl GameLoop for DefaultGameLoop {
    fn update<F>(&mut self, context: &mut Context, update_function: F) -> LoopResult
    where
        F: FnMut(&mut Context),
    {

    }

    fn render<F>(&mut self, context: &mut Context, render_function: F) -> LoopResult
    where
        F: FnMut(&mut Context),
    {
    }
}

pub struct DefaultGameLoopBuilder {
    game_loop: DefaultGameLoop,
}

impl DefaultGameLoopBuilder {
    pub fn new() -> Self {
        Self {
            game_loop: DefaultGameLoop::new(),
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

    pub fn build(self) -> DefaultGameLoop {
        self.game_loop
    }
}

#[cfg(test)]
mod default_game_loop_test {
    use super::*;
    use crate::core::Context;
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

    #[test_case(120.0, 30 => 4  ; "4 times at 120 tps and 30 fps")]
    #[test_case(120.0, 60 => 2  ; "2 times at 120 tps and 60 fps")]
    #[test_case(120.0, 120 => 1 ; "1 time at 120 tps and 120 fps")]
    fn should_tick(tick_rate: f64, fps: u64) -> u64 {
        let mut context = Context;
        let mut game_loop = DefaultGameLoopBuilder::new()
            .with_tps(tick_rate)
            .build();

        let mut ticks = 0;
        game_loop.lag = Duration::from_millis(1000 / fps);
        game_loop.update(&mut context, move |_| {
            ticks += 1;
        });

        ticks
    }

    fn lag_test_game_loop(lag: u64) -> DefaultGameLoop {
        let mut game_loop = DefaultGameLoopBuilder::new().build();
        game_loop.lag = Duration::from_millis(lag);
        game_loop
    }
}

    #[cfg(test)]
mod default_game_loop_builder_tests {
    use super::*;

    #[test]
    fn should_have_default_values_in_builder() {
        let game_loop = DefaultGameLoopBuilder::new().build();

        assert_eq!(game_loop.tps(), 120.0);
        assert_eq!(game_loop.max_update_time(), Duration::from_millis(100));
    }

    #[test]
    fn should_have_tps_setter() {
        let game_loop = DefaultGameLoopBuilder::new().with_tps(60.0).build();

        assert_eq!(game_loop.tps(), 60.0);
    }

    #[test]
    fn should_have_max_update_time_setter() {
        let game_loop = DefaultGameLoopBuilder::new()
            .with_max_update_time(Duration::from_secs(1))
            .build();

        assert_eq!(game_loop.max_update_time(), Duration::from_secs(1));
    }
}
