/// Provides the events used by the window API.
#[non_exhaustive]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum WindowEvent {}

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

    #[cfg(test)]
    /// A test event only used by unit tests.
    Test,
}

#[cfg(test)]
mod event_tests {
    use super::*;

    #[test]
    fn should_implement_clone() {
        let event = Event::EventsCleared;
        let clone = event.clone();
        assert_eq!(event, clone);
    }

    #[test]
    fn should_implement_copy() {
        let event = Event::EventsCleared;
        let copy = copy_test(event);
        assert_eq!(event, copy);
    }

    fn copy_test(event: Event) -> Event {
        event
    }
}
