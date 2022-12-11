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
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct WindowId {
    id: usize,
}

impl WindowId {
    pub fn new() -> Self {
        Self {
            id: read_and_incrament_next_id(),
        }
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
