use std::marker::PhantomData;

use wolf_engine::framework::plugins::*;
use wolf_engine::framework::FrameworkBuilder;
use wolf_engine::prelude::*;

// Just a test resource used by our plugin.
pub struct MyResource(String);

pub struct MyPlugin<E: UserEvent> {
    // Because plugins have a generic type, we need to include `PhantomData`, or the compiler will
    // complain.
    _event_type: PhantomData<E>,
}

impl<E: UserEvent> MyPlugin<E> {
    pub fn new() -> Self {
        Self {
            _event_type: PhantomData,
        }
    }
}

impl<E: UserEvent> Plugin<E> for MyPlugin<E> {
    fn name(&self) -> &str {
        // Return a name to identify the plugin in logs.
        "Test Plugin"
    }

    fn load(&mut self, builder: &mut FrameworkBuilder<E>) -> PluginResult {
        // Plugins can add resources to the engine.
        builder.with_resource(MyResource("Hello, world!".to_string()));

        Ok(())
    }
}

pub fn main() {
    let (_event_loop, context) = wolf_engine::framework::init::<()>()
        .with_plugin(MyPlugin::new()) // Plugins are added at startup.
        .build()
        .unwrap();

    // Resources added by plugins can be accessed just like any other resource.
    let plugin_resource = context.resources().get::<MyResource>().unwrap();

    println!("{}", plugin_resource.0);
}
