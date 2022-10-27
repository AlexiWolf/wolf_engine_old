use crate::events::EventLoop;

pub trait Context<E>: EventLoop<E> {}

impl<T, E> Context<E> for T where T: EventLoop<E> {}

