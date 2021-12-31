mod state_machine;
mod transition;

pub use state_machine::*;
pub use transition::*;

use crate::Context;

pub type UpdateResult = Option<Transition>;

pub type RenderResult = ();

pub trait State {
    fn update(&mut self, context: &mut Context) -> UpdateResult;

    fn render(&mut self, context: &mut Context) -> RenderResult;
}

