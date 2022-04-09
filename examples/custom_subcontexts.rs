use log::*;
use wolf_engine::*;

pub fn main() {
    #[cfg(feature = "logging")]
    wolf_engine::logging::initialize_logging(LevelFilter::Info);

    EngineBuilder::new()
        .with_subcontext(CustomContext::new("Hello, World!"))
        .build()
        .run(Box::from(MyState));
}

pub struct CustomContext {
    pub message: String,
    pub count: u32,
}

impl CustomContext {
    pub fn new(message: &str) -> Self {
        Self {
            message: message.to_string(),
            count: 0,
        }
    }
}

impl Subcontext for CustomContext {}

pub struct MyState;

impl State for MyState {
    fn update(&mut self, context: &mut Context) -> OptionalTransition {
        let mut custom_context = context.get_mut::<CustomContext>().unwrap();
        if custom_context.count == 10 {
            Some(Transition::Quit)
        } else {
            custom_context.count += 1;
            None
        }
    }

    fn render(&mut self, context: &mut Context) -> RenderResult {
        let custom_context = context.get::<CustomContext>().unwrap();
        info!("{}: {}", custom_context.message, custom_context.count);
    }
}
