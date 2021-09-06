use std::time::Duration;

pub type TicksPerSecond = f64;

pub struct DefaultGameLoop;

impl DefaultGameLoop {
    pub fn tps(&self) -> TicksPerSecond {
        0.0
    }

    pub fn max_update_time(&self) -> Duration {
        Duration::default()
    }
}

pub struct DefaultGameLoopBuilder;

impl DefaultGameLoopBuilder {
    pub fn new() -> Self {
        Self
    }

    pub fn build(self) -> DefaultGameLoop {
        DefaultGameLoop
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
