
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
