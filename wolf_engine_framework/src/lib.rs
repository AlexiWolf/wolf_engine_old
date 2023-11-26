//! Provides a complete, batteries-included framework for Wolf Engine.
//!
//! The framework module provides a high-level framework for quickly and easily building games. It
//! makes several decisions, and handles a lot of the heavy lifting for you, so you can focus
//! more on building your game.  The framework includes a plugin system, and game state /
//! state-stack architecture.

mod framework_builder;
pub use framework_builder::*;

pub mod plugins;

use wolf_engine_core::events::UserEvent;

pub fn init<E: UserEvent>() -> FrameworkBuilder<E> {
    FrameworkBuilder::<E>::new()
}

#[cfg(test)]
mod framework_init_tests {
    pub struct TestResourceA;
    pub struct TestResourceB;

    #[test]
    fn should_add_resources() {
        let (_event_loop, context) = crate::init::<()>()
            .with_resource(TestResourceA)
            .with_resource(TestResourceB)
            .build()
            .unwrap();
        assert!(
            context.resources().get::<TestResourceA>().is_some(),
            "Resource insertion failed"
        );
        assert!(
            context.resources().get::<TestResourceB>().is_some(),
            "Resource insertion failed"
        );
    }
}
