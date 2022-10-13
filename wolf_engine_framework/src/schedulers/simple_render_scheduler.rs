use wolf_engine_core::*;

use crate::schedulers::RenderScheduler;
use crate::stages::*;
use crate::*;

/// A simple [RenderScheduler] which immediately runs [State::render()] just once.
#[derive(Debug)]
pub struct SimpleRenderScheduler;

impl RenderScheduler for SimpleRenderScheduler {
    fn render(
        &mut self,
        context: &mut Context,
        state: &mut dyn State,
        stage_callbacks: &mut StageCallbacks,
    ) {
        stage_callbacks.run(StageType::PreRender, context);
        stage_callbacks.run(StageType::Render, context);
        state.render(context);
        stage_callbacks.run(StageType::PostRender, context);
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
            scheduler.render(&mut context, &mut state, &mut StageCallbacks::new());
        }
    }

    #[test]
    fn should_run_engine_stages() {
        scheduler_integration_tests::should_run_render_stages(SimpleRenderScheduler);
    }
}
