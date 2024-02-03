use crate::events::Event;

/// Provides the main events used by Wolf Engine.
#[non_exhaustive]
#[derive(Event, Debug, PartialEq, Eq)]
pub enum EngineEvent {
    /// Emitted when the engine should quit.
    Quit,

    /// Indicates the end of a frame.
    ///
    /// `EventsCleared` should be emitted only after all other events have been processed.
    EventsCleared,
}
