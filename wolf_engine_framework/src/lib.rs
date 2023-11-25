//! Provides a complete, batteries-included framework for Wolf Engine.
//!
//! The framework module provides a high-level framework for quickly and easily building games. It
//! makes several decisions, and handles a lot of the heavy lifting for you, so you can focus
//! more on building your game.  The framework includes a plugin system, and game state /
//! state-stack architecture.

#[cfg(test)]
mod framework_tests {
    #[test]
    fn should_extend_default_init() {
        let engine = crate::init()
            .build();
    }
}
