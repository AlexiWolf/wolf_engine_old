mod state_machine;

use std::fmt::Display;

pub use state_machine::*;

use crate::Context;

pub type UpdateResult = ();

pub type RenderResult = ();

pub trait State: Display {
    fn update(&mut self, context: &mut Context) -> UpdateResult;

    fn render(&mut self, context: &mut Context) -> RenderResult;
}

