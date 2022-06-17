use crate::*;

pub struct EngineContext {}

impl Subcontext for EngineContext {}

#[cfg(test)]
mod engine_context_tests {
    use super::EngineContext;

    use super::EngineContext;

    pub fn should_initialize_has_quit_to_false() {
        let engine_context = EngineContext::new();

        assert!(!engine_context.has_quit, "has_quit should be false.");
    }
}
