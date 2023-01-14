use std::sync::mpsc::Sender;

use crate::{EngineControls, prelude::Event};

pub struct Context<'a, D> {
    pub data: D,
    event_sender: &'a Sender<Event>,
    has_quit: bool,
}

impl<'a, D> Context<'a, D> {
    pub fn new(event_sender: &Sender<Event>, data: D) -> Self {
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

impl<'a, D> EngineControls for Context<'a, D> {
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
