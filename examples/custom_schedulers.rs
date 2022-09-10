use log::info;
use wolf_engine::schedulers::*;
use wolf_engine::*;

pub fn main() {
    #[cfg(feature = "logging")]
    {
        logging::initialize_logging(log::LevelFilter::Info);
    }

    EngineBuilder::new()
        .with_update_scheduler(Box::from(CustomUpdateScheduler))
        .with_render_scheduler(Box::from(CustomRenderScheduler))
        .build()
        .expect("Failed to create the Engine")
        .run(Box::from(MainState));
}

pub struct CustomUpdateScheduler;

impl UpdateScheduler for CustomUpdateScheduler {
    fn update(&mut self, context: &mut Context, state: &mut dyn State) {
        info!("Hello from a custom Update Scheduler!");
        state.update(context);
    }
}

pub struct CustomRenderScheduler;

impl RenderScheduler for CustomRenderScheduler {
    fn render(&mut self, context: &mut Context, state: &mut dyn State) {
        info!("Hello from a custom Render Scheduler!");
        state.render(context);
    }
}

pub struct MainState;

impl State for MainState {
    fn update(&mut self, _context: &mut Context) -> Transition {
        info!("Update");
        None
    }

    fn render(&mut self, _context: &mut Context) {
        info!("Render");
    }
}
