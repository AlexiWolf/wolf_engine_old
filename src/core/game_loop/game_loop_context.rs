use crate::{Frames, Ticks};

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
/// 
/// Once created, the GameLoopContext exposes information about the [GameLoop].
/// 
/// ```
/// # use wolf_engine::GameLoopContext;
/// #
/// # let game_loop_context = GameLoopContext::new();
/// #
/// game_loop_context.ticks();
/// game_loop_context.frames();
/// ```
/// 
/// Tick and frame information can be added to the context.  
/// 
/// **Note:** These method are only intended for the [GameLoop] and other parts of the engine. If
/// you are not providing a custom game loop, you **should not** touch these.
/// 
/// ```
/// # use wolf_engine::GameLoopContext;
/// #
/// # let game_loop_context = GameLoopContext::new();
/// #
/// # assert_eq!(game_loop_context.ticks(), 0, "There should be 0 ticks before add_tick is called");
/// # assert_eq!(game_loop_context.frames(), 0, "There should be 0 frames before add_tick is called");
/// #
/// game_loop_context.add_tick();
/// game_loop_context.add_frame();
/// #
/// # game_loop_context.ticks();
/// # game_loop_context.frames();
/// #
/// # assert_eq!(game_loop_context.ticks(), 1, "1 tick should have been added");
/// # assert_eq!(game_loop_context.frames(), 1, "1 frame should have been added");
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