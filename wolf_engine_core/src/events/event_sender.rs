pub trait EventSender<E> {
    fn send_event(&self, event: E) -> Result<(), String>;
}

pub trait EventSenderProxy<E>: EventSender<E> + Send + Sync {}

impl<T, E> EventSenderProxy<E> for T where T: EventSender<E> + Clone + Send + Sync {}
