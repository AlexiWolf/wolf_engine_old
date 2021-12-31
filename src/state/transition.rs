use crate::State;

pub enum Transition {
    Push(Box<dyn State>),
    Pop,
    Quit,
}
