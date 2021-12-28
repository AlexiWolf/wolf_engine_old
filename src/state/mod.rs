mod state_machine;
mod transition;

use std::fmt::Display;

pub use state_machine::*;
pub use transition::*;

use crate::Context;

pub type UpdateResult = Option<Transition>;

pub type RenderResult = ();

pub trait State: Display {
    fn update(&mut self, context: &mut Context) -> UpdateResult;

    fn render(&mut self, context: &mut Context) -> RenderResult;
}
