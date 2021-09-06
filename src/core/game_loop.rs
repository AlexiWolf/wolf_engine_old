pub struct GameLoopContext;

pub struct GameLoopContextBuilder;

impl GameLoopContextBuilder {
    pub fn new() -> Self {
        Self
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

        assert_eq!(context.tps(), 120);
        assert_eq!(context.max_update_time(), Duration::from_millis(100));
    }
}