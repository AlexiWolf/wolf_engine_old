use crate::EngineControls;

pub struct Context<D> {
    pub data: D,
}

impl<D> From<D> for Context<D> {
    fn from(data: D) -> Self {
        Self { data }
    }
}

impl<D> EngineControls for Context<D> {
    fn quit(&self) {
    }

    fn has_quit(&self) -> bool {
        false
    }

    fn update(&self) {
    }

    fn render(&self) {
    }
}
