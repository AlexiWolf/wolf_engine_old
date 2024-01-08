//! Provides an event system for the engine.
//!
//! Wolf Engine re-exports [Generic Event Queue](generic_event_queue), see the original crate for
//! details.

use core::fmt::Debug;

use downcast_rs::*;
pub use generic_event_queue::*;
pub use wolf_engine_codegen::Event;

mod event;
pub use event::*;
mod event_loop;
pub use event_loop::*;

/// An alias to the main [`EventSender`] type associated with the 
/// [`EventLoop`].
pub type MainEventSender = mpsc::MpscEventSender<Box<dyn Event>>;

/// Represents a [`Boxed`](Box) dynamic [`Event`].
pub type EventBox = Box<dyn Event>;

/// A dynamically-typed event.
///
/// Events can be downcasted back to their original type using the [`Downcast`] trait.
pub trait Event: Downcast + Debug + 'static {}
impl_downcast!(Event);

#[cfg(test)]
mod event_tests {
    use test_case::test_case;

    use super::*;

    #[derive(Event, Debug)]
    struct TestEvent(&'static str);

    #[test_case(&TestEvent("Hello, World!"))]
    fn should_auto_impl_event(event: &dyn Event) {
        if let Some(event) = event.downcast_ref::<TestEvent>() {
            assert_eq!(event.0, "Hello, World!");
        }
    }
}
