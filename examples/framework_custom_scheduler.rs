use log::info;

use wolf_engine::framework::schedulers::*;
use wolf_engine::framework::stages::*;
use wolf_engine::prelude::*;

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

#[derive(Debug)]
pub struct CustomUpdateScheduler;

impl UpdateScheduler for CustomUpdateScheduler {
    fn update(
        &mut self,
        context: &mut Context,
        state: &mut dyn State,
        stage_callbacks: &mut StageCallbacks,
    ) {
        info!("Hello from a custom Update Scheduler!");
        stage_callbacks.run(StageType::PreUpdate, context);
        stage_callbacks.run(StageType::Update, context);
        state.update(context);
        stage_callbacks.run(StageType::PostUpdate, context);
    }
}

#[derive(Debug)]
pub struct CustomRenderScheduler;

impl RenderScheduler for CustomRenderScheduler {
    fn render(
        &mut self,
        context: &mut Context,
        state: &mut dyn State,
        stage_callbacks: &mut StageCallbacks,
    ) {
        info!("Hello from a custom Render Scheduler!");
        stage_callbacks.run(StageType::PreRender, context);
        stage_callbacks.run(StageType::Render, context);
        state.render(context);
        stage_callbacks.run(StageType::PostRender, context);
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
