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
pub struct GameLoopContext<'a> {
    game_loop_info: &'a GameLoopInfo
}

impl<'a> GameLoopContext<'a> {
    pub fn from_game_loop_info(game_loop_info: &'a GameLoopInfo) -> Self {
        Self {
            game_loop_info
        }
    }

    pub fn ticks(&self) -> Ticks {
        self.game_loop_info.ticks()
    }
    

    pub fn frames(&self) -> Frames {
        self.game_loop_info.frames()
    }
}