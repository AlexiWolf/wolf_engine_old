use std::sync::{Arc, Mutex};

use lazy_static::lazy_static;

lazy_static! {
    static ref NEXT_ID: Arc<Mutex<usize>> = Arc::from(Mutex::from(0));
}

fn read_and_incrament_next_id() -> usize {
    let id = *NEXT_ID.lock().unwrap();
    *NEXT_ID.lock().unwrap() = id + 1;
    id
}

/// Provides a unique id for keeping track of a [`Window`](crate::Window).
///
/// All new window IDs are guaranteed to be unique.  A window ID is only equal to copies of itself.
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct WindowId {
    id: usize,
}

impl WindowId {
    /// Create a new, unique window id.
    pub fn new() -> Self {
        Self {
            id: read_and_incrament_next_id(),
        }
    }
}

impl Default for WindowId {
    fn default() -> Self {
        Self::new()
    } 
}

#[cfg(test)]
mod window_id_tests {
    use super::*;

    #[test]
    fn should_be_unique() {
        let a = WindowId::new();
        let b = WindowId::new();

        assert_ne!(a, b);
    }

    #[test]
    fn should_be_equal_to_self() {
        let original = WindowId::new();
        let clone = original.clone();

        assert_eq!(original, clone);
    }
}
