//! Provides a plugin system for the engine.

use crate::FrameworkBuilder;

/// A result type for the plugin system.
pub type PluginResult = Result<(), String>;

/// A module which adds new functionality to the engine.
pub trait Plugin {
    /// Returns a people-friendly name for the plugin.
    ///
    /// This is mostly used to identify the plugin in logs.  There aren't any specific requirements
    /// for what the name should be, and it may change, so you probably shouldn't use this to
    /// uniquely identify plugins.
    fn name(&self) -> &str;

    /// Loads the plugin using the provided [`FrameworkBuilder`].
    ///
    /// The plugin does all setup here.
    ///
    /// **Note:** Plugins shouldn't try to load other plugins using the builder.  At this point in
    /// the setup process, it's not possible to add additional plugins.  Nothing will happen if you
    /// try.
    fn load(&mut self, builder: &mut FrameworkBuilder) -> PluginResult;
}

pub(crate) struct PluginLoader {
    plugins: Vec<Box<dyn Plugin>>,
}

impl PluginLoader {
    pub fn new() -> Self {
        Self {
            plugins: Vec::new(),
        }
    }

    pub fn add_plugin(&mut self, plugin: Box<dyn Plugin + 'static>) {
        self.plugins.push(plugin);
    }

    pub fn load_plugins(&mut self, builder: &mut FrameworkBuilder) -> PluginResult {
        for plugin in &mut self.plugins {
            match plugin.load(builder) {
                Ok(_) => (),
                Err(error) => {
                    let error_message =
                        format!("Error loading Plugin ({}): {}", plugin.name(), error);
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

    pub struct TestResource;

    pub struct TestPlugin {
        should_fail: bool,
    }

    impl TestPlugin {
        pub fn new(should_fail: bool) -> Self {
            Self { should_fail }
        }
    }

    impl Plugin for TestPlugin {
        fn load(&mut self, builder: &mut FrameworkBuilder) -> PluginResult {
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
        let (_event_loop, context) = crate::init()
            .with_plugin(TestPlugin::new(false))
            .build()
            .unwrap();
        assert!(
            context.resources().get::<TestResource>().is_ok(),
            "Resource insertion failed"
        );
    }

    #[test]
    fn should_handle_plugin_failures() {
        let result = crate::init().with_plugin(TestPlugin::new(true)).build();
        assert!(result.is_err(), "The build should have failed");
    }
}
