use crate::events::EventLoop;

/// A marker trait indicating which types can be used as context-data on the [`Engine`].
///
/// Thit trait is automatically implemented for all types meeting the type constraints, so you
/// don't normally need to implement it yourself.
///
/// To be used as a context, a type needs to:
///
/// - Implement [`Eventloop`].
pub trait Context<E>: EventLoop<E> {}

impl<T, E> Context<E> for T where T: EventLoop<E> {}
