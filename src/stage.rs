use std::fmt::Debug;

use crate::Context;

#[cfg(test)]
use mockall::automock;

/// A collection of [Callbacks](Callback) to run during a specific [StageType].
pub type Stage = Vec<Box<dyn Callback>>;

#[cfg_attr(test, automock)]
pub trait Callback {
    fn run(&mut self, context: &mut Context);
}

/// Represents an [Engine] stage.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum StageType {
    /// Runs before the [Update stage](Stage::Update) has started.
    PreUpdate,

    /// Runs main [State] and [Engine] update logic.
    Update,

    /// Runs after the [Update stage](Stage::Update) has finished. 
    PostUpdate,
   
    /// Runs before the [Render stage](Stage::Render) has started.
    PreRender,

    /// Runs main [State] and [Engine] render logic.
    Render,

    /// Runs after the [Render stage](Stage::Render) has finished.
    PostRender,
}

#[derive(Default)]
pub struct StageCallbacks {
    pre_update : Stage,
    update: Stage,
    post_update: Stage,
    pre_render: Stage,
    render: Stage,
    post_render: Stage,
}

impl StageCallbacks {
    pub fn new() -> Self {
        Self {
            pre_update: Vec::new(),
            update: Vec::new(),
            post_update: Vec::new(),
            pre_render: Vec::new(),
            render: Vec::new(),
            post_render: Vec::new(),
        }
    }

    pub fn push(&mut self, stage: StageType, callback: Box<dyn Callback>) {
        self.get_mut(stage)
            .push(callback);
    }

    pub fn run(&mut self, stage: StageType, context: &mut Context) {
        self.get_mut(stage)
            .iter_mut()
            .for_each(|callback| { callback.run(context); });
    }

    pub fn get(&self, stage: StageType) -> &Stage {
        match stage {
            StageType::PreUpdate => &self.pre_update, 
            StageType::Update => &self.update,
            StageType::PostUpdate => &self.post_update,
            StageType::PreRender => &self.pre_render,
            StageType::Render => &self.render,
            StageType::PostRender => &self.post_render,
        }
    }

    pub fn get_mut(&mut self, stage: StageType) -> &mut Stage {
        match stage {
            StageType::PreUpdate => &mut self.pre_update, 
            StageType::Update => &mut self.update,
            StageType::PostUpdate => &mut self.post_update,
            StageType::PreRender => &mut self.pre_render,
            StageType::Render => &mut self.render,
            StageType::PostRender => &mut self.post_render,
        }
    }
}

impl Debug for StageCallbacks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("StageCallbacks")
            .field("pre_update_callbacks", &self.pre_update.len())
            .field("update_callbacks", &self.update.len())
            .field("post_update_callbacks", &self.post_update.len())
            .field("pre_render_callbacks", &self.pre_render.len())
            .field("render_callbacks", &self.render.len())
            .field("post_render_callbacks", &self.post_render.len())
            .finish()
    }
}

#[cfg(test)]
mod stage_tests {
    use super::*;

    use test_case::test_case;

    #[test]
    fn should_create_empty_stage_callbacks() {
        let stage_callbacks = StageCallbacks::new();

        assert_eq!(stage_callbacks.pre_update.len(), 0);
        assert_eq!(stage_callbacks.update.len(), 0);
        assert_eq!(stage_callbacks.post_update.len(), 0);
        assert_eq!(stage_callbacks.pre_render.len(), 0);
        assert_eq!(stage_callbacks.render.len(), 0);
        assert_eq!(stage_callbacks.post_render.len(), 0);
    }

    #[test]
    fn should_implement_default() {
        let _stage_callbacks = StageCallbacks::default();
    }

    #[test]
    fn should_implement_debug() {
        let stage_callbacks = StageCallbacks::new();
        println!("{:#?}", stage_callbacks); 
    }
    
    #[test_case(StageType::PreUpdate)]
    #[test_case(StageType::Update)]
    #[test_case(StageType::PostUpdate)]
    #[test_case(StageType::PreRender)]
    #[test_case(StageType::Render)]
    #[test_case(StageType::PostRender)]
    fn should_add_function_with_correct_callback_group(stage: StageType) {
        let mut stage_callbacks = StageCallbacks::new();
        let callback = MockCallback::new();
        stage_callbacks.push(stage, Box::from(callback));

        assert_eq!(1, stage_callbacks.get(stage).len(), "The callback was not added to the stage");
        assert_eq!(1, stage_callbacks.get_mut(stage).len(), "The callback was not added to the stage");
    }
    
    #[test_case(StageType::PreUpdate)]
    #[test_case(StageType::Update)]
    #[test_case(StageType::PostUpdate)]
    #[test_case(StageType::PreRender)]
    #[test_case(StageType::Render)]
    #[test_case(StageType::PostRender)]
    fn should_run_stage_callbacks(stage: StageType) {
        let mut stage_callbacks = StageCallbacks::new();
        let mut context = Context::new();
        let mut callback = MockCallback::new();
        callback.expect_run().once().return_const(());
        stage_callbacks.push(stage, Box::from(callback));

        stage_callbacks.run(stage, &mut context);
    }
    
}

#[cfg(test)]
pub mod scheduler_integration_tests {
    use super::*;
    use crate::*;
    use crate::schedulers::*;

    pub fn should_run_update_stages<U: 'static + UpdateScheduler>(update_scheduler: U) {
        let mut engine = test_engine(
            Box::from(update_scheduler),
            Box::from(SimpleRenderScheduler));
        push_callback(&mut engine.stage_callbacks, StageType::PreUpdate);
        push_callback(&mut engine.stage_callbacks, StageType::Update); 
        push_callback(&mut engine.stage_callbacks, StageType::PostUpdate);

        engine.update();
    }

    pub fn should_run_render_stages<R: 'static + RenderScheduler>(render_scheduler: R) {
        let mut engine = test_engine(
            Box::from(FixedUpdateScheduler::default()),
            Box::from(render_scheduler));
        push_callback(&mut engine.stage_callbacks, StageType::PreRender);
        push_callback(&mut engine.stage_callbacks, StageType::Render); 
        push_callback(&mut engine.stage_callbacks, StageType::PostRender);

        engine.render();
    }

    fn test_engine(
        update_scheduler: Box<dyn UpdateScheduler>,
        render_scheduler: Box<dyn RenderScheduler>,
    ) -> Engine {
        Engine::builder()
            .with_update_scheduler(Box::from(update_scheduler))
            .with_render_scheduler(Box::from(render_scheduler))
            .build()
            .expect("Failed to build the Engine")
    }

    fn push_callback(stage_callbacks: &mut StageCallbacks, stage: StageType) {
        let mut callback = MockCallback::new();
        callback.expect_run().times(1..).return_const(());
        stage_callbacks.push(stage, Box::from(callback)); }
}
