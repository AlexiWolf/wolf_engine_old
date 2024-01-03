use std::sync::Arc;

use super::EventSender;

/// Provides the events used by the window API.
#[non_exhaustive]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum WindowEvent {}

/// An alias to the main [`EventSender`] type associated with the [`EventLoop`](crate::EventLoop).
pub type MainEventSender = Arc<dyn EventSender<Event>>;

/// A user-defined [`Event`] type.
pub trait UserEvent: PartialEq + Clone + Copy + 'static {}

impl<T> UserEvent for T where T: PartialEq + Clone + Copy + 'static {}

/// Provides the main events used by Wolf Engine.
#[non_exhaustive]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Event {
    /// Emitted when the engine should quit.
    Quit,

    /// Indicates the end of a frame.
    ///
    /// `EventsCleared` should be emitted only after all other events have been processed.
    EventsCleared,

    /// A [`WindowEvent`] emitted by the window system.
    WindowEvent(WindowEvent),
}

#[cfg(test)]
mod event_tests {
    use super::*;

    #[test]
    fn should_implement_clone() {
        let event: Event = Event::EventsCleared;
        let clone = event.clone();
        assert_eq!(event, clone);
    }

    #[test]
    fn should_implement_copy() {
        let event: Event = Event::EventsCleared;
        let copy = copy_test(event);
        assert_eq!(event, copy);
    }

    fn copy_test(event: Event) -> Event {
        event
    }
}
