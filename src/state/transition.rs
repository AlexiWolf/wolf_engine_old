use crate::State;

/// Indicates if / how the [StateStack](crate::StateStack) should change [State].
pub type OptionalTransition = Option<Transition>;

/// Indicates the type of [Transition] the [StateStack](crate::StateStack) should
/// perform.
pub enum Transition {
    
    Push(Box<dyn State>),

    /// Pop the active [State] off the stack and shut it down.
    Pop,

    /// Pop all [State]s off the stack, then push a new state.
    CleanPush(Box<dyn State>),

    /// Pop all [State]s off the stack, then shut down the engine.
    Quit,
}
