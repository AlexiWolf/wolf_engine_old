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
    use super::shared_test_fixtures::*;

    #[test]
    fn should_quit() {
        let mut engine = engine_with_empty_state();
        
        engine.context.quit();
         
        assert_engine_is_not_running(&engine);
    }

    #[test]
    fn should_only_indicate_has_quit_after_quit_has_been_called() {
        let mut engine = engine_with_empty_state();

        assert_has_quit_indicates_quit_has_been_called(&mut engine.context);
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
    use super::shared_test_fixtures::*;

    #[test]
    fn should_quit() {
        let mut engine = engine_with_empty_state();
        
        engine.quit();

        assert_engine_is_not_running(&engine);
    }

    #[test]
    fn should_only_indicate_has_quit_after_quit_has_been_called() {
        let mut engine = engine_with_empty_state();
        
        assert_has_quit_indicates_quit_has_been_called(&mut engine);
    }
}

#[cfg(test)]
mod shared_test_fixtures {
    use super::*;

    pub fn engine_with_empty_state() -> Engine {
        let mut engine = Engine::default();
        engine.state_stack.push(Box::from(EmptyState), &mut engine.context);
        engine
    }

    pub fn assert_engine_is_not_running(engine: &Engine) {
        assert!(!engine.is_running(), "The engine is running, but it should not be.");
    }

    pub fn assert_has_quit_indicates_quit_has_been_called(controls: &mut dyn EngineControls) {
        assert!(!controls.has_quit(), "The engine should not have quit yet");

        controls.quit();

        assert!(controls.has_quit(), "The engine should have quit, but it didn't.");
    }
}
