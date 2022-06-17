
#[cfg(test)]
mod engine_controls_test {
    use super::*;
    use crate::*;

    #[test]
    fn should_quit() {
        let mut engine = Engine::default();

        engine.context.quit();
         
        assert!(!engine.is_running(), "The engine is running, but it should not be.");
    }
}
