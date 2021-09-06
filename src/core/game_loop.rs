use std::time::Duration;

pub struct GameLoopContext {
    tps: f64,
    max_update_time: Duration
}

impl GameLoopContext {
    pub fn tps(&self) -> f64 {
        self.tps
    }

    pub fn max_update_time(&self) -> Duration {
        self.max_update_time
    }
}

impl Default for GameLoopContext {
    fn default() -> Self {
        Self { tps: 120.0, max_update_time: Duration::from_millis(100) }
    }
}

pub struct GameLoopContextBuilder {
    context: GameLoopContext
}

impl GameLoopContextBuilder {
    pub fn new() -> Self {
        Self {
            context: GameLoopContext::default()
        }
    }

    pub fn build(self) -> GameLoopContext {
        self.context
    }

    pub fn with_tps(mut self, tps: f64) -> Self {
        self.context.tps = tps;
        self
    }
}

#[cfg(test)]
mod game_loop_builder_tests {
    use super::*;

    #[test]
    fn should_have_default_settings() {
        let context = GameLoopContextBuilder::new()
            .build();

        assert_eq!(context.tps(), 120.0);
        assert_eq!(context.max_update_time(), Duration::from_millis(100));
    }

    #[test]
    fn should_allow_custom_tps() {
        let context = GameLoopContextBuilder::new()
            .with_tps(60.0)
            .build();

        assert_eq!(context.tps(), 60.0);
    }
}