use crate::State;

/// Instructs the [StateMachine] on what to do.
pub type Transition = Option<Signal>;

pub enum Signal {
    Push(Box<dyn State>),
    CleanPush(Box<dyn State>),
    Pop,
    Quit,
}
