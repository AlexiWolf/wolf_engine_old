use crate::State;

pub enum Transition {
    ToState(Box<dyn State>),
    Pop,
    Quit,
}
