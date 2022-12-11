/// Provides the events used by the window API.
#[non_exhaustive]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum WindowEvent {}

/// Provides the main events used by the [`Engine`](crate::Engine).
#[non_exhaustive]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Event {

    /// Emitted when the engine should quit.
    Quit,

    /// Indicates the engine should update game logic.
    Update,

    /// Indicates the engine should render the game.
    Render,

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
        let event = Event::Update;
        let clone = event.clone();
        assert_eq!(event, clone);
    }

    #[test]
    fn should_implement_copy() {
        let event = Event::Update;
        let copy = copy_test(event);
        assert_eq!(event, copy);
    }

    fn copy_test(event: Event) -> Event {
        event
    }
}
