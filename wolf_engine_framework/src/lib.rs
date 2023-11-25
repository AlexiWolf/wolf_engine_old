//! Provides a complete, batteries-included framework for Wolf Engine.
//!
//! The framework module provides a high-level framework for quickly and easily building games. It
//! makes several decisions, and handles a lot of the heavy lifting for you, so you can focus
//! more on building your game.  The framework includes a plugin system, and game state /
//! state-stack architecture.

use wolf_engine_core::EngineBuilder;
use wolf_engine_core::events::UserEvent;

pub struct FrameworkBuilder<E: UserEvent> {}

impl<E: UserEvent> FrameworkBuilder<E> {

}

pub fn init<E: UserEvent>() -> EngineBuilder<E> {
    wolf_engine_core::init()
}

pub mod plugins {
    use crate::FrameworkBuilder; 
    use wolf_engine_core::events::UserEvent;
    
    pub type PluginResult = Result<(), String>;

    pub trait Plugin {
        fn load<E: UserEvent>(&mut self, builder: &mut FrameworkBuilder<E>) -> PluginResult;
    }
}

#[cfg(test)]
mod framework_tests {
    use super::*;
    use super::plugins::*;

    pub struct TestPlugin;

    impl Plugin for TestPlugin {
        fn load<E: UserEvent>(&mut self, builder: &mut FrameworkBuilder<E>) -> PluginResult {
            Ok(())
        }
    }

    #[test]
    fn should_extend_default_init() {
        let _engine = crate::init::<()>()
            .with_plugin(TestPlugin)
            .build();
    }
}
