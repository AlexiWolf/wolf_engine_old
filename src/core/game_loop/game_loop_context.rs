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
/// let a = GameLoopInfo::new();
/// let b = a.clone();
/// 
/// # assert_eq!(a.ticks(), 0);
/// # assert_eq!(a.frames(), 0);
/// # assert_eq!(b.ticks(), 0)
/// # assert_eq!(b.frames(), 0)
/// ```
#[derive(Clone, Copy)]
pub struct GameLoopInfo;

impl GameLoopInfo {
    pub fn new() -> Self {
        Self
    }
}
