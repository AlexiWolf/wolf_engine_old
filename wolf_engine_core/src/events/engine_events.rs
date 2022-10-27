#[derive(Debug, PartialEq, Eq)]
pub enum Event {
    Quit,
    Update,
    Render,
    EventsCleared,
}

