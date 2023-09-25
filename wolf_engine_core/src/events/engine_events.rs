/// Provides the events used by the window API.
#[non_exhaustive]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum WindowEvent {}

pub trait UserEvent {}

impl<T> UserEvent for T {}

/// Provides the main events used by Wolf Engine.
#[non_exhaustive]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Event<E: UserEvent> {
    /// Emitted when the engine should quit.
    Quit,

    /// Indicates the end of a frame.
    ///
    /// `EventsCleared` should be emitted only after all other events have been processed.
    EventsCleared,

    /// A [`WindowEvent`] emitted by the window system.
    WindowEvent(WindowEvent),

    UserDefined(E),

    #[cfg(test)]
    /// A test event only used by unit tests.
    Test,
}

#[cfg(test)]
mod event_tests {
    use super::*;

    #[test]
    fn should_implement_clone() {
        let event: Event<()> = Event::EventsCleared;
        let clone = event.clone();
        assert_eq!(event, clone);
    }

    #[test]
    fn should_implement_copy() {
        let event: Event<()> = Event::EventsCleared;
        let copy = copy_test(event);
        assert_eq!(event, copy);
    }

    fn copy_test<E: UserEvent>(event: Event<E>) -> Event<E> {
        event
    }

    #[test]
    fn should_support_user_defined_events() {
        let _event = Event::UserDefined("custom_event");
    }
}
