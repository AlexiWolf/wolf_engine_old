//! Provides an event system for the engine.
//!
//! Wolf Engine re-exports [Generic Event Queue](generic_event_queue), see the original crate for
//! details.

use core::fmt::Debug;

use downcast_rs::*;
pub use generic_event_queue::*;

mod event;
pub use event::*;

pub trait EventTrait: Downcast + Debug + 'static {}
impl_downcast!(EventTrait);

impl<T> EventTrait for T where T: Debug + 'static {}

#[cfg(test)]
mod event_tests {
    use test_case::test_case;

    use super::*;

    #[derive(Debug)]
    struct TestEvent(&'static str);
    
    #[test_case(&TestEvent("Hello, World!"))]
    fn should_auto_impl_event(event: &dyn EventTrait) {
        if let Some(event) = event.downcast_ref::<TestEvent>() {
            assert_eq!(event.0, "Hello, World!");
        }
    }
}
