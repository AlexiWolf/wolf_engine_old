use std::sync::{Arc, Mutex};

use crate::{Frames, Ticks};

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
/// let b = &a;
/// 
/// # assert_eq!(a.ticks(), 0);
/// # assert_eq!(a.frames(), 0);
/// ```
pub struct GameLoopInfo {
    ticks: Arc<Mutex<Ticks>>,
    frames: Arc<Mutex<Frames>>
}

impl GameLoopInfo {
    pub fn new() -> Self {
        Self {
            ticks: Arc::from(Mutex::from(0)),
            frames: Arc::from(Mutex::from(0))
        }
    }

    fn ticks(&self) -> Ticks {
        self.ticks.lock().expect("Failed to unlock ticks")
    }

    fn frames(&self) -> Frames {
        self.frames.lock().expect("Failed to unlock frames")
    }
}
