//! Provides a complete, batteries-included framework for Wolf Engine.
//!
//! The framework module provides a high-level framework for quickly and easily building games. It
//! makes several decisions, and handles a lot of the heavy lifting for you, so you can focus
//! more on building your game.  The framework includes a plugin system, and game state /
//! state-stack architecture.

use plugins::Plugin;
use wolf_engine_core::ecs::ResourcesBuilder;
use wolf_engine_core::ecs::systems::Resource;
use wolf_engine_core::{EngineBuilder, Engine};
use wolf_engine_core::events::UserEvent;

pub struct FrameworkBuilder<E: UserEvent> {
    resource_builder: ResourcesBuilder,
    plugins: Vec<Box<dyn Plugin<E>>>,
}

impl<E: UserEvent> FrameworkBuilder<E> {
    pub(crate) fn new(engine_builder: EngineBuilder<E>) -> Self {
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
    FrameworkBuilder::<E>::new(wolf_engine_core::init())
}

pub mod plugins {
    use crate::FrameworkBuilder; 
    use wolf_engine_core::events::UserEvent;
    
    pub type PluginResult<E> = Result<FrameworkBuilder<E>, String>;

    pub trait Plugin<E: UserEvent> {
        fn load(&mut self, builder: FrameworkBuilder<E>) -> PluginResult<E>;
    }

    #[cfg(test)]
    mod plugin_loader_tests {
        use super::*;
        use crate::FrameworkBuilder;

        use std::marker::PhantomData;

        use wolf_engine_core::events::UserEvent;

        pub struct TestResource;

        pub struct TestPlugin<E: UserEvent> {
            _phantom: PhantomData<E>,
        }

        impl<E: UserEvent> TestPlugin<E> {
            pub fn new() -> Self {
                Self {
                    _phantom: PhantomData::default(), 
                }
            }
        }

        impl<E: UserEvent> Plugin<E> for TestPlugin<E> {
            fn load(&mut self, builder: FrameworkBuilder<E>) -> PluginResult<E> {
                Ok(
                    builder.with_resource(TestResource)
                )
            }
        }

        #[test]
        fn should_load_plugins() {
            let (_event_loop, context) = crate::init::<()>()
                .with_plugin(TestPlugin::new())
                .build();
            assert!(context.resources().get::<TestResource>().is_some(), "Resource insertion failed");
        }
    }
}

#[cfg(test)]
mod framework_tests {
    pub struct TestResource;

    #[test]
    fn should_extend_default_init() {
        let (_event_loop, context) = crate::init::<()>()
            .with_resource(TestResource)
            .build();
        assert!(context.resources().get::<TestResource>().is_some(), "Resource insertion failed");
    }
}
