use crate::State;

/// Indicates if / how the [StateMachine](crate::StateMachine) should change [State].
pub type Transition = Option<TransitionType>;

/// Indicates the type of [Transition] the [StateMachine](crate::StateMachine) should 
/// perform.
pub enum TransitionType {

    /// Push a new [State] to the top of the stack.
    Push(Box<dyn State>),
    
    /// Pop the active [State] off the stack and shut it down.
    Pop,

    /// Pop all [State]s off the stack, then push a new state.
    CleanPush(Box<dyn State>),

    /// Pop all [State]s off the stack, then shut down the engine.
    Quit,
}
