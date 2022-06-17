use crate::{*, contexts::EngineContext};

pub trait EngineControls {
    fn quit(&mut self);
    fn has_quit(&self) -> bool;
}

impl EngineControls for Context {
    fn quit(&mut self) {
        let mut engine_context = self.borrow_mut::<EngineContext>()
            .expect("There is no EngineContext");
        engine_context.has_quit = true;
    }

    fn has_quit(&self) -> bool {
        let engine_context = self.borrow::<EngineContext>().unwrap();
        engine_context.has_quit
    }
}

#[cfg(test)]
mod engine_controls_test {
    use super::*;

    #[test]
    fn should_quit() {
        let mut engine = Engine::default();
        engine.state_stack.push(Box::from(EmptyState), &mut engine.context);
        
        engine.context.quit();
         
        assert!(!engine.is_running(), "The engine is running, but it should not be.");
    }
}
