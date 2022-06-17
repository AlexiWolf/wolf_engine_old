use crate::*;

pub struct EngineContext {
    pub has_quit: bool,
}

impl EngineContext {
    pub fn new() -> Self {
        Self {
            has_quit: false,
        }
    }
}

impl Subcontext for EngineContext {}

#[cfg(test)]
mod engine_context_tests {
    use super::*;
    
    #[test]
    pub fn should_initialize_has_quit_to_false() {
        let engine_context = EngineContext::new();

        assert!(!engine_context.has_quit, "has_quit should be false.");
    }
}
