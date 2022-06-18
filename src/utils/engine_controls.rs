use crate::{*, contexts::EngineContext};


/// An extension trait which provides basic control functions for the [Engine]. 
pub trait EngineControls {
    /// Shutdown the [Engine] and exit.
    fn quit(&mut self);

    /// Returns true if [EngineControls::quit()] has been called.
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
mod engine_controls_context_implementation_tests {
    use super::*;

    #[test]
    fn should_quit() {
        let mut engine = Engine::default();
        engine.state_stack.push(Box::from(EmptyState), &mut engine.context);
        
        engine.context.quit();
         
        assert!(!engine.is_running(), "The engine is running, but it should not be.");
    }

    #[test]
    fn should_only_return_true_from_has_quit_when_quit_has_been_called() {
        let mut engine = Engine::default();
        engine.state_stack.push(Box::from(EmptyState), &mut engine.context);

        assert!(!engine.context.has_quit(), "The engine should not have quit yet");

        engine.context.quit();

        assert!(engine.context.has_quit(), "The engine should have quit, but it didn't.");
    }
}


impl EngineControls for Engine {
    fn quit(&mut self) {
        self.context.quit(); 
    }

    fn has_quit(&self) -> bool {
        self.context.has_quit()
    }
}

#[cfg(test)]
mod engine_controls_engine_implementation_tests {
    use super::*;

    #[test]
    fn should_quit() {
        let mut engine = Engine::default();
        engine.state_stack.push(Box::from(EmptyState), &mut engine.context);

        engine.quit();

        assert!(!engine.is_running(), "The engine is running, but it should not be.");
    }

    #[test]
    fn should_only_return_true_from_has_quit_when_quit_has_been_called() {
        let mut engine = Engine::default();
        engine.state_stack.push(Box::from(EmptyState), &mut engine.context);

        assert!(!engine.has_quit(), "The engine should not have quit yet");

        engine.quit();

        assert!(engine.has_quit(), "The engine should have quit, but it didn't.");
    }
}
