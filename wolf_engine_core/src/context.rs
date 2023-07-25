use std::sync::Arc;

use crate::events::*;

/// TODO: Rework Context documentation.
pub struct Context<D> {
    /// The user-facing engine data.  Normally things like subsystems.
    pub data: D,
    event_sender: Arc<dyn EventSender<Event>>,
}

impl<D> Context<D> {
    /// Create a new `Context` from the provided [`EventQueue`] and data.
    pub fn new(event_queue: &dyn EventQueue<Event>, data: D) -> Self {
        Self {
            data,
            event_sender: event_queue.event_sender(),
        }
    }

    pub fn quit(&self) {
        self.event_sender.send_event(Event::Quit).ok();
    }

    pub fn update(&self) {
        self.event_sender.send_event(Event::Update).ok();
    }

    pub fn render(&self) {
        self.event_sender.send_event(Event::Render).ok();
    }
}

impl<D> HasEventSender<Event> for Context<D> {
    fn event_sender(&self) -> Arc<dyn EventSender<Event>> {
        self.event_sender.clone()
    }
}
