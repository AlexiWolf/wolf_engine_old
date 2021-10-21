use crate::GameLoopContext;

/// Provides access to all shared engine state.
pub struct Context<'a> {
    pub game_loop: GameLoopContext<'a>
}
