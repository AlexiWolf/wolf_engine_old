use crate::events::Event;

/// Provides the main events used by Wolf Engine.
#[non_exhaustive]
#[derive(Event, Debug, PartialEq, Eq, Clone, Copy)]
pub enum EngineEvent {
    /// Emitted when the engine should quit.
    Quit,

    /// Indicates the end of a frame.
    ///
    /// `EventsCleared` should be emitted only after all other events have been processed.
    EventsCleared,
}

#[cfg(test)]
mod event_tests {
    use super::*;

    #[test]
    fn should_implement_clone() {
        let event: EngineEvent = EngineEvent::EventsCleared;
        let clone = event.clone();
        assert_eq!(event, clone);
    }

    #[test]
    fn should_implement_copy() {
        let event: EngineEvent = EngineEvent::EventsCleared;
        let copy = copy_test(event);
        assert_eq!(event, copy);
    }

    fn copy_test(event: EngineEvent) -> EngineEvent {
        event
    }
}
