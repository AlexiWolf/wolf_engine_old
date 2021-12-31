mod state_machine;
mod transition;

use std::fmt::Display;

pub use state_machine::*;
pub use transition::*;

use crate::Context;

pub type UpdateResult = Option<Transition>;

pub type RenderResult = ();

pub trait State {
    fn update(&mut self, context: &mut Context) -> UpdateResult;

    fn render(&mut self, context: &mut Context) -> RenderResult;
}

impl Display for dyn State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "")
    }
}

