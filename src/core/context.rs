use crate::GameLoopContext;

/// Provides access to all shared engine state.
pub struct Context {
    pub game_loop: GameLoopContext,
}

pub struct ContextBuilder;

impl ContextBuilder {
    pub fn new() -> Self {
        Self
    }

    pub fn build(self) -> Context {
        Context {
            game_loop: GameLoopContext::new()
        }
    }
}