/// Provides access to information and controls for the [GameLoop](crate::GameLoop).
/// 
/// # Examples
/// 
/// The only way to create a game loop context is from [GameLoop](crate::GameLoop) instance.
/// 
/// ```
/// # use wolf_engine::{GameLoopContext, FixedUpdateGameLoopBuilder65};
/// #
/// # let game_loop = FixedUpdateGameLoopBuilder::new().build();
/// let game_loop_context = GameLoopContext::from_loop(&game_loop);
/// ```
pub struct GameLoopContext;