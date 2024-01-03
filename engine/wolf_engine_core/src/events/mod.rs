//! Provides an event system for the engine.
//!
//! Wolf Engine re-exports [Generic Event Queue](generic_event_queue), see the original crate for
//! details.

use downcast_rs::*;
pub use generic_event_queue::*;

mod event;
pub use event::*;

pub trait EventTrait: Downcast + 'static {}
impl_downcast!(EventTrait);

impl<T> EventTrait for T where T: 'static {}

#[cfg(test)]
mod event_tests {
    use test_case::test_case;

    use super::*;

    struct TestEvent(&'static str);
    
    #[test_case(&TestEvent("Hello, World!"))]
    fn should_auto_impl_event(event: &dyn EventTrait) {
        if let Some(event) = event.downcast_ref::<TestEvent>() {
            assert_eq!(event.0, "Hello, World!");
        }
    }
}
