use std::sync::mpsc::Sender;

use crate::{EngineControls, prelude::Event};

pub struct Context<D> {
    pub data: D,
    event_sender: Sender<Event>,
    has_quit: bool,
}

impl<D> Context<D> {
    pub fn new(event_sender: Sender<Event>, data: D) -> Self {
        Self {
            data,
            event_sender,
            has_quit: false,
        }
    }

    pub(crate) fn set_has_quit(&mut self, has_quit: bool) {
        self.has_quit = has_quit;
    }
}

impl<D> EngineControls for Context<D> {
    fn quit(&self) {
        
    }

    fn has_quit(&self) -> bool {
        self.has_quit
    }

    fn update(&self) {
    }

    fn render(&self) {
    }
}
