//! Provides a complete, batteries-included framework for Wolf Engine.
//!
//! The framework module provides a high-level framework for quickly and easily building games. It
//! makes several decisions, and handles a lot of the heavy lifting for you, so you can focus
//! more on building your game.  The framework includes a plugin system, and game state /
//! state-stack architecture.

pub mod plugins;

use plugins::Plugin;

use wolf_engine_core::Engine;
use wolf_engine_core::ecs::ResourcesBuilder;
use wolf_engine_core::ecs::systems::Resource;
use wolf_engine_core::events::UserEvent;

pub struct FrameworkBuilder<E: UserEvent> {
    resource_builder: ResourcesBuilder,
    plugins: Vec<Box<dyn Plugin<E>>>,
}

impl<E: UserEvent> FrameworkBuilder<E> {
    pub(crate) fn new() -> Self {
        Self {
            plugins: Vec::new(),
            resource_builder: ResourcesBuilder::default(),
        }
    }

    pub fn with_plugin<P: Plugin<E> + 'static>(mut self, plugin: P) -> Self {
        self.plugins.push(Box::from(plugin));  
        self
    }

    pub fn with_resource<T: Resource + 'static>(mut self, resource: T)-> Self {
        self.resource_builder.add_resource(resource);
        self
    }

    pub fn build(mut self) -> Engine<E> {
        let plugins = std::mem::take(&mut self.plugins);

        for mut plugin in plugins {
            self = plugin.load(self).expect("Failed to load plugin");
        }
        
        wolf_engine_core::init()
            .with_resources(self.resource_builder)
            .build()
    }
}

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
            .build();
        assert!(context.resources().get::<TestResourceA>().is_some(), "Resource insertion failed");
        assert!(context.resources().get::<TestResourceB>().is_some(), "Resource insertion failed");
    }
}
