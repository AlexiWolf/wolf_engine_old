use std::time::{Duration, Instant};

fn main() {

}

pub struct Timer {
    current: Instant,
    previous: Instant,
    lag: Duration,
    updates_per_second: u32
}

impl Timer {
    pub fn new() -> Self {
        let now = Instant::now();
        Self {
            current: now,
            previous: now,
            lag: Duration::from_secs(0),
            updates_per_second: 120
        }
    }

    pub fn with_updates_per_second(updates_per_second: u32) -> Self {
        let now = Instant::now();
        Self {
            current: now,
            previous: now,
            lag: Duration::from_secs(0),
            updates_per_second
        }
    }

    pub fn run() {
        
    }
}

impl Default for Timer {
    fn default() -> Self {
        Self::new()
    }
}
