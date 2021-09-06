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

    pub fn build(self) -> DefaultGameLoop {
        self.game_loop
    }
}

#[cfg(test)]
mod default_game_loop_tests {
    use super::*;

    #[test]
    fn should_have_default_values_in_builder() {
        let game_loop = DefaultGameLoopBuilder::new()
            .build();

        assert_eq!(game_loop.tps(), 120.0);
        assert_eq!(game_loop.max_update_time(), Duration::from_millis(100))
    }
}
