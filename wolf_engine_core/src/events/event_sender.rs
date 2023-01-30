pub trait EventSender<E> {
    fn send_event(&self, event: E) -> Result<(), String>;
}

pub trait EventSenderProxy<E>: EventSender<E> + Send + Sync {}
