pub trait EventLoop<E> {
    fn next_event(&self) -> Option<E>;
    fn send_event(&self, event: E);
}

