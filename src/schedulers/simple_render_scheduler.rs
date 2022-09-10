use crate::schedulers::RenderScheduler;
use crate::*;

/// A simple [RenderScheduler] which immediately runs [State::render()] just once.
pub struct SimpleRenderScheduler;

impl RenderScheduler for SimpleRenderScheduler {
    fn render(&mut self, context: &mut Context, state: &mut dyn State) {
        state.render(context);
    }
}

#[cfg(test)]
mod simple_render_scheduler_tests {
    use super::*;

    #[test]
    fn should_run_state_render_once_per_call() {
        let mut scheduler = SimpleRenderScheduler;
        let mut context = Context::new();
        let mut state = MockState::new();
        state.expect_render().times(5).return_const(());

        for _ in 0..5 {
            scheduler.render(&mut context, &mut state);
        }
    }
}
