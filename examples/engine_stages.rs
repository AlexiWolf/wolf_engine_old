use wolf_engine::*;
use wolf_engine::stages::*;

pub fn main() {
    logging::initialize_logging(log::LevelFilter::Debug);
    Engine::builder()
        .with_subcontext(ExampleContext::default())
        .with_stage_callback_fn(StageType::Update, increment_by_1)
        .build()
        .expect("Failed to build the engine")
        .run(Box::from(MainState));
}

pub fn increment_by_1(context: &mut Context) {
    let mut subcontext = context.borrow_mut::<ExampleContext>().unwrap();
    subcontext.number += 1;
}

pub struct MainState;

impl State for MainState {
    fn update(&mut self,context: &mut Context) -> Transition {
        let subcontext = context.borrow::<ExampleContext>().unwrap();
        log::info!("{}", subcontext.number);
        if subcontext.number >= 10 {
            Some(TransitionType::Clean)
        } else {
            None
        }
    }

    fn render(&mut self, _context: &mut Context) {}
}

#[derive(Default)]
pub struct ExampleContext {
    pub number: u32,
}

impl Subcontext for ExampleContext {}
