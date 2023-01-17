use std::sync::Arc;

use crate::{
    events::EventSender,
    prelude::{Event, EventLoop},
    EngineControls,
};

pub struct Context<D> {
    pub data: D,
    event_sender: Arc<dyn EventSender<Event>>,
    has_quit: bool,
}

impl<D> Context<D> {
    pub fn new(event_loop: &dyn EventLoop<Event>, data: D) -> Self {
        Self {
            data,
            event_sender: event_loop.sender(),
            has_quit: false,
        }
    }

    pub(crate) fn set_has_quit(&mut self, has_quit: bool) {
        self.has_quit = has_quit;
    }
}

impl<D> EngineControls for Context<D> {
    fn quit(&self) {
        self.event_sender.send(Event::Quit).ok();
    }

    fn has_quit(&self) -> bool {
        self.has_quit
    }

    fn update(&self) {
        self.event_sender.send(Event::Update).ok();
    }

    fn render(&self) {
        self.event_sender.send(Event::Render).ok();
    }
}
