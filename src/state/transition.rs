use crate::State;

/// Instructs the [StateMachine] on what to do.
pub type Transition = Option<TransitionType>;

pub enum TransitionType {
    Push(Box<dyn State>),
    CleanPush(Box<dyn State>),
    Pop,
    Quit,
}
