use log::*;
use wolf_engine::prelude::*;

pub fn main() {
    #[cfg(feature = "logging")]
    logging::initialize_logging(LevelFilter::Debug);

    EngineBuilder::new()
        .with_plugin(Box::from(MessagePlugin::new("Hello, world!")))
        .with_plugin(Box::from(FailurePlugin))
        .build()
        .expect("Failed to build the engine")
        .run(Box::from(GameState));
}

pub struct MessagePlugin {
    message: String,
}

impl Plugin for MessagePlugin {
    fn setup(&mut self, engine_builder: EngineBuilder) -> PluginResult {
        Ok(engine_builder.with_subcontext(MessageContext::new(self.message.clone())))
    }
}

impl MessagePlugin {
    pub fn new(message: &str) -> Self {
        Self {
            message: message.to_string(),
        }
    }
}

pub struct FailurePlugin;

impl Plugin for FailurePlugin {
    fn setup(&mut self, engine_builder: EngineBuilder) -> PluginResult {
        debug!("Intentionally returning an error.");
        Err(("Something isn't right!", engine_builder))
    }
}

pub struct MessageContext {
    message: String,
}

impl MessageContext {
    pub fn new(message: String) -> Self {
        Self { message }
    }
}

impl Subcontext for MessageContext {}

pub struct GameState;

impl State for GameState {
    fn update(&mut self, context: &mut Context) -> Transition {
        let message = context.borrow::<MessageContext>().unwrap();
        info!("{}", message.message);
        Some(TransitionType::Clean)
    }

    fn render(&mut self, _context: &mut Context) {}
}
