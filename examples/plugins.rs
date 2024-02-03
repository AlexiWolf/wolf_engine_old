use wolf_engine::framework::plugins::*;
use wolf_engine::framework::FrameworkBuilder;

// Just a test resource used by our plugin.
pub struct MyResource(String);

pub struct MyPlugin {}

impl MyPlugin {
    pub fn new() -> Self {
        Self {}
    }
}

impl Plugin for MyPlugin {
    fn name(&self) -> &str {
        // Return a name to identify the plugin in logs.
        "Test Plugin"
    }

    fn load(&mut self, builder: &mut FrameworkBuilder) -> PluginResult {
        // Plugins can add resources to the engine.
        builder.with_resource(MyResource("Hello, world!".to_string()));

        Ok(())
    }
}

pub fn main() {
    let (_event_loop, context) = wolf_engine::framework::init()
        .with_plugin(MyPlugin::new()) // Plugins are added at startup.
        .build()
        .unwrap();

    // Resources added by plugins can be accessed just like any other resource.
    let plugin_resource = context.resources().get::<MyResource>().unwrap();

    println!("{}", plugin_resource.0);
}
