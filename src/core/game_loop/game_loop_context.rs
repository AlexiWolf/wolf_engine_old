use std::sync::{Arc, Mutex};

use crate::{Frames, Ticks};

/// Safely stores information about a [GameLoop](create::GameLoop).
///
/// The main idea is the GameLoopInfo acts as a bridge between the [GameLoop](create::GameLoop), and the
/// [GameLoopContext] instance.  It uses interior mutability to safely allow the game loop to update the information 
/// while while the context reads from it.
///
/// # Examples
///
/// The game loop info object can be immutably borrowed and shared freely.
///
/// ```
/// # use wolf_engine::GameLoopInfo;
/// #
/// let a = GameLoopInfo::new();
/// let b = &a;
///
/// # assert_eq!(a.ticks(), 0, "The tick count does not start at 0");
/// # assert_eq!(a.frames(), 0, "The frame count does not start at 0");
/// #
/// a.add_tick();
/// a.add_frame();
///
/// assert_eq!(b.ticks(), 1, "Changes in a's tick count are not reflected in b");
/// assert_eq!(b.frames(), 1, "Changes in a's frame count are not reflected in b");
/// ```
pub struct GameLoopInfo {
    ticks: Arc<Mutex<Ticks>>,
    frames: Arc<Mutex<Frames>>,
}

impl GameLoopInfo {
    pub fn new() -> Self {
        Self {
            ticks: Arc::from(Mutex::from(0)),
            frames: Arc::from(Mutex::from(0)),
        }
    }

    pub fn ticks(&self) -> Ticks {
        *self.ticks.lock().expect("Failed to unlock ticks")
    }

    pub fn frames(&self) -> Frames {
        *self.frames.lock().expect("Failed to unlock frames")
    }

    pub fn add_tick(&self) {
        *self.ticks.lock().expect("Failed to unlock ticks") += 1
    }

    pub fn add_frame(&self) {
        *self.frames.lock().expect("Failed to unlock frames") += 1
    }
}
