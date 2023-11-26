use crate::FrameworkBuilder;

use wolf_engine_core::events::UserEvent;

pub type PluginResult<E> = Result<FrameworkBuilder<E>, String>;

pub trait Plugin<E: UserEvent> {
    fn load(&mut self, builder: FrameworkBuilder<E>) -> PluginResult<E>;
}

pub(crate) struct PluginLoder<E: UserEvent> {
    plugins: Vec<Box<dyn Plugin<E>>>,
}

impl<E: UserEvent> PluginLoder<E> {
    pub fn new() -> Self {
        Self { plugins: Vec::new() }
    }

    pub fn add_plugin(&mut self, plugin: Box<dyn Plugin<E> + 'static>) {
        self.plugins.push(plugin); 
    }

    pub fn load_plugins(&mut self, mut builder: FrameworkBuilder<E>) -> PluginResult<E> {
        for plugin in &mut self.plugins {
            builder = plugin.load(builder).expect("Failed to load plugin");
        }
        Ok(builder)
    }
}

#[cfg(test)]
mod plugin_loader_tests {
    use super::*;

    use std::marker::PhantomData;

    use wolf_engine_core::events::UserEvent;

    pub struct TestResource;

    pub struct TestPlugin<E: UserEvent> {
        _event_type: PhantomData<E>,
    }

    impl<E: UserEvent> TestPlugin<E> {
        pub fn new() -> Self {
            Self {
                _event_type: PhantomData,
            }
        }
    }

    impl<E: UserEvent> Plugin<E> for TestPlugin<E> {
        fn load(&mut self, builder: FrameworkBuilder<E>) -> PluginResult<E> {
            Ok(builder.with_resource(TestResource))
        }
    }

    #[test]
    fn should_load_plugins() {
        let (_event_loop, context) = crate::init::<()>().with_plugin(TestPlugin::new()).build();
        assert!(
            context.resources().get::<TestResource>().is_some(),
            "Resource insertion failed"
        );
    }
}
