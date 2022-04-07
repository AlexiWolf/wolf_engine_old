use log::*;
use wolf_engine::*;

pub fn main() {
    #[cfg(feature = "logging")]
    logging::initialize_logging(LevelFilter::Info);

    EngineBuilder::new()
        .with_plugin(Box::from(MessagePlugin::new("Hello, world!")))
        .build()
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
    fn update(&mut self, context: &mut Context) -> OptionalTransition {
        let message = context.get::<MessageContext>().unwrap();
        info!("{}", message.message);
        Some(Transition::Quit)
    }

    fn render(&mut self, _context: &mut Context) -> RenderResult {}
}
