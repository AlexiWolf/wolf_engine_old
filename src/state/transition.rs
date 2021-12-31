use crate::State;

pub type Transition = Option<Signal>;

pub enum Signal {
    Push(Box<dyn State>),
    CleanPush(Box<dyn State>),
    Pop,
    Quit,
}
