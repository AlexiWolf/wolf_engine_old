use std::time::Duration;

pub struct GameLoopContext {
    tps: f64,
    max_update_time: Duration
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
        GameLoopContext
    }
}

#[cfg(test)]
mod game_loop_tests {
    use super::*;

    #[test]
    fn should_have_default_settings() {
        let context = GameLoopContextBuilder::new()
            .build();

        assert_eq!(context.tps(), 120.0);
        assert_eq!(context.max_update_time(), Duration::from_millis(100));
    }
}