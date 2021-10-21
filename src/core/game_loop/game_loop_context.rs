/// Safely stores information about a [GameLoop](create::GameLoop).
/// 
/// The main idea is the GameLoopInfo acts as a bridge between the [GameLoop](create::GameLoop), and the
/// [GameLoopContext] instance.  It uses interior mutability with an [Arc], and a [Mutex] to safely allow the game loop
/// to update the information while it's context reads from it.
/// 
/// # Examples
/// 
/// The game loop info object can be copied, but the copy will still have the same internal values as the other copies.
/// 
/// ```
/// # use wolf_engine::GameLoopInfo;
/// #
/// let info = GameLoopInfo::new();
/// ```
pub struct GameLoopInfo;
