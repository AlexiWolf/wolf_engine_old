//! Provides a plugin system for the engine.

use crate::FrameworkBuilder;

use wolf_engine_core::events::UserEvent;

pub type PluginResult = Result<(), String>;

pub trait Plugin<E: UserEvent> {
    fn name(&self) -> &str;
    fn load(&mut self, builder: &mut FrameworkBuilder<E>) -> PluginResult;
}

pub(crate) struct PluginLoader<E: UserEvent> {
    plugins: Vec<Box<dyn Plugin<E>>>,
}

impl<E: UserEvent> PluginLoader<E> {
    pub fn new() -> Self {
        Self {
            plugins: Vec::new(),
        }
    }

    pub fn add_plugin(&mut self, plugin: Box<dyn Plugin<E> + 'static>) {
        self.plugins.push(plugin);
    }

    pub fn load_plugins(&mut self, builder: &mut FrameworkBuilder<E>) -> PluginResult {
        for plugin in &mut self.plugins {
            match plugin.load(builder) {
                Ok(_) => (),
                Err(error) => {
                    let error_message = format!(
                        "Error loading Plugin ({}): {}",
                        plugin.name(),
                        error
                    );
                    log::error!("{}", error_message);
                    return Err(error);
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod plugin_loader_tests {
    use super::*;

    use std::marker::PhantomData;

    use wolf_engine_core::events::UserEvent;

    pub struct TestResource;

    pub struct TestPlugin<E: UserEvent> {
        should_fail: bool,
        _event_type: PhantomData<E>,
    }

    impl<E: UserEvent> TestPlugin<E> {
        pub fn new(should_fail: bool) -> Self {
            Self {
                should_fail,
                _event_type: PhantomData,
            }
        }
    }

    impl<E: UserEvent> Plugin<E> for TestPlugin<E> {
        fn load(&mut self, builder: &mut FrameworkBuilder<E>) -> PluginResult {
            builder.with_resource(TestResource);
            if self.should_fail {
                Err("Nah, I don't really feel like it.  Why don't you ask me later?".to_string())
            } else {
                Ok(())
            }
        }

        fn name(&self) -> &str {
            "Test Plugin"
        }
    }

    #[test]
    fn should_load_plugins() {
        let (_event_loop, context) = crate::init::<()>()
            .with_plugin(TestPlugin::new(false))
            .build()
            .unwrap();
        assert!(
            context.resources().get::<TestResource>().is_some(),
            "Resource insertion failed"
        );
    }

    #[test]
    fn should_handle_plugin_failures() {
        let result = crate::init::<()>()
            .with_plugin(TestPlugin::new(true))
            .build();
        assert!(result.is_err(), "The build should have failed");
    }
}
