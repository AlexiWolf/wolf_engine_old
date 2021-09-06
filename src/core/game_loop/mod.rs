mod default_game_loop;

use crate::core::Context;

pub use default_game_loop::*;

pub type LoopResult = ();

pub trait GameLoop {
    fn update(
        &mut self,
        context: &mut Context,
        update_function: fn(&mut Context)
    ) -> LoopResult;

    fn render(
        &mut self,
        context: &mut Context,
        render_function: fn(&mut Context)
    ) -> LoopResult;
}
