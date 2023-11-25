//! Provides a complete, batteries-included framework for Wolf Engine.
//!
//! The framework module provides a high-level framework for quickly and easily building games. It
//! makes several decisions, and handles a lot of the heavy lifting for you, so you can focus
//! more on building your game.  The framework includes a plugin system, and game state /
//! state-stack architecture.

use wolf_engine_core::EngineBuilder;
use wolf_engine_core::events::UserEvent;

pub fn init<E: UserEvent>() -> EngineBuilder<E> {
    wolf_engine_core::init()
}

#[cfg(test)]
mod framework_tests {

    pub struct TestPlugin;

    impl Plugin for TestPlugin {
        
    }

    #[test]
    fn should_extend_default_init() {
        let _engine = crate::init::<()>()
            .with_plugin(TestPlugin)
            .build();
    }
}
