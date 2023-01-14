use crate::EngineControls;

pub struct Context<D> {
    pub data: D,
    has_quit: bool,
}

impl<D> Context<D> {
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
