use wolf_engine_core::*;

/// Provides game-accessible [Engine] state and controls.
pub struct EngineContext {
    /// Indicates if the engine has quit.
    pub has_quit: bool,
}

impl EngineContext {
    pub fn new() -> Self {
        Self { has_quit: false }
    }
}

impl Subcontext for EngineContext {}

impl Default for EngineContext {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod engine_context_tests {
    use super::*;

    #[test]
    fn should_initialize_has_quit_to_false() {
        let engine_context = EngineContext::new();

        assert!(!engine_context.has_quit, "has_quit should be false.");
    }
}
