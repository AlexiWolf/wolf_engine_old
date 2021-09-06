use std::time::Duration;

pub type TicksPerSecond = f64;

pub struct DefaultGameLoop {
    tps: TicksPerSecond,
    max_update_time: Duration
}

impl DefaultGameLoop {
    pub fn new() -> Self {
        Self {
            tps: 120.0,
            max_update_time: Duration::from_millis(100)
        }
    }

    pub fn tps(&self) -> TicksPerSecond {
        self.tps
    }

    pub fn max_update_time(&self) -> Duration {
        self.max_update_time
    }
}

pub struct DefaultGameLoopBuilder {
    game_loop: DefaultGameLoop
}

impl DefaultGameLoopBuilder {
    pub fn new() -> Self {
        Self {
            game_loop: DefaultGameLoop::new()
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
    use std::time::Instant;

    #[test]
    fn should_have_consistent_tick_rate() {
        let mut last_update: Option<Instant> = None;
        let mut ticks_made = 0;
        let context = Context;
        let game_loop = DefaultGameLoopBuilder::new()
            .build();

        thead.sleep(Duration::from_millis(1000 / 30));
        game_loop.update(context, |context| {
            if let Some(last_update) = last_update {
                assert_eq!(last_update.elapsed().as_millis(), 8);
            } else {
                last_update = Some(Instant::now());
            }
            ticks_made += 1;
        });

        assert_eq!(ticks_made, 4);
    }
}

#[cfg(test)]
mod default_game_loop_builder_tests {
    use super::*;

    #[test]
    fn should_have_default_values_in_builder() {
        let game_loop = DefaultGameLoopBuilder::new()
            .build();

        assert_eq!(game_loop.tps(), 120.0);
        assert_eq!(game_loop.max_update_time(), Duration::from_millis(100));
    }

    #[test]
    fn should_have_tps_setter() {
        let game_loop = DefaultGameLoopBuilder::new()
            .with_tps(60.0)
            .build();

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
