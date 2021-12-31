mod state_machine;
mod transition;

pub use state_machine::*;
pub use transition::*;

use crate::Context;

#[cfg(test)]
use mockall::automock;

pub type UpdateResult = Option<Transition>;

pub type RenderResult = ();

#[cfg_attr(test, automock)]
pub trait State {
    fn update(&mut self, context: &mut Context) -> UpdateResult;

    fn render(&mut self, context: &mut Context) -> RenderResult;
}
