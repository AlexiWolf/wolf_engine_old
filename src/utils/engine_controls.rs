use crate::*;

pub trait EngineControls {
    fn quit(&mut self);
}

impl EngineControls for Context {
    fn quit(&mut self) {
        
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
