mod default_game_loop;

use crate::core::Context;

pub use default_game_loop::*;

pub type LoopResult = ();

pub trait GameLoop {
    fn update<F>(
        &mut self,
        context: &mut Context,
        update_function: F
    ) -> LoopResult
    where F: FnMut(&mut Context);

    fn render<F>(
        &mut self,
        context: &mut Context,
        render_function: F
    ) -> LoopResult
    where F: FnMut(&mut Context);
}
