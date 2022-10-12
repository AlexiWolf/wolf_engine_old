use log::*;

use wolf_engine::*;
use wolf_engine::framework::*;

pub fn main() {
    #[cfg(feature = "logging")]
    wolf_engine::logging::initialize_logging(LevelFilter::Info);

    EngineBuilder::new()
        .with_subcontext(CustomContext::new("Hello, World!"))
        .build()
        .expect("Failed to build the engine")
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
    fn update(&mut self, context: &mut Context) -> Transition {
        let mut custom_context = context.borrow_mut::<CustomContext>().unwrap();
        if custom_context.count == 10 {
            Some(TransitionType::Clean)
        } else {
            custom_context.count += 1;
            None
        }
    }

    fn render(&mut self, context: &mut Context) {
        let custom_context = context.borrow::<CustomContext>().unwrap();
        info!("{}: {}", custom_context.message, custom_context.count);
    }
}
