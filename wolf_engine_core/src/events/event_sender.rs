pub trait EventSender<E> {
    fn send(&self, event: E) -> Result<(), String>;
}

pub trait ThreadSafeEventSender<E>: EventSender<E> + Send + Sync {}

impl<T, E> ThreadSafeEventSender<E> for T where T: EventSender<E> + Send + Sync {}
