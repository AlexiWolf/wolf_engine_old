mod fixed_update_game_loop;

use crate::core::Context;

pub use fixed_update_game_loop::*;

pub type LoopResult = ();

pub trait GameLoop {
    fn update<F>(&mut self, context: &mut Context, update_function: F) -> LoopResult
    where
        F: FnMut(&mut Context) -> LoopResult;

    fn render<F>(&mut self, context: &mut Context, render_function: F) -> LoopResult
    where
        F: FnMut(&mut Context) -> LoopResult;
}
