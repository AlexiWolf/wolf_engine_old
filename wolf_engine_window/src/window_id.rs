use std::sync::{Arc, Mutex};

use lazy_static::lazy_static;

lazy_static! {
    static ref NEXT_ID: Arc<Mutex<usize>> = Arc::from(Mutex::from(0));
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct WindowId {}

impl WindowId {
    pub fn new() -> Self {
        Self {}
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
}
