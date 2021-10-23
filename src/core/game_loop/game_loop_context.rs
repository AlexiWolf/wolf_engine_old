use crate::{Frames, GameLoopInfo, Ticks};

/// Provides access to information and controls for the [GameLoop](crate::GameLoop).
/// 
/// # Examples
/// 
/// The GameLoopContext can be created directly using the new method.
/// 
/// ```
/// # use wolf_engine::GameLoopContext;
/// #
/// let game_loop_context = GameLoopContext::new();
/// ```
pub struct GameLoopContext;

impl GameLoopContext {
    pub fn new() -> Self {
        Self
    }

    pub fn ticks(&self) -> Ticks {
        0
    }
    

    pub fn frames(&self) -> Frames {
        0
    }
}