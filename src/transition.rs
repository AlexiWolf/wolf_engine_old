use crate::State;

/// An [Optional](Option), [TransitionType] used to send instructions to the [StateStack](crate::StateStack).
pub type Transition = Option<TransitionType>;

/// Represents a state change for the [StateStack](crate::StateStack) to perform.
pub enum TransitionType {
    /// Push a new [State] to the top of the stack.
    Push(Box<dyn State>),

    /// Pop the active [State] off the stack and shut it down.
    Pop,

    /// Pop all [State]s off the stack, then push a new state.
    CleanPush(Box<dyn State>),

    /// Pop all [State]s off the stack, then shut down the engine.
    Clean,
}
