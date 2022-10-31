/// Provides the main events used by the [`Engine`](crate::Engine).
#[derive(Debug, PartialEq, Eq)]
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
}
