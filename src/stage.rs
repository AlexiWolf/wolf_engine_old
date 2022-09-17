use crate::Context;

pub type StageCallback = fn(&mut Context);

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Stage {
    PreUpdate,
    Update,
    PostUpdate,
    PreRender,
    Render,
    PostRender,
}

#[derive(Default)]
pub struct StageCallbacks {
    pre_update : Vec<StageCallback>,
    update: Vec<StageCallback>,
    post_update: Vec<StageCallback>,
    pre_render: Vec<StageCallback>,
    render: Vec<StageCallback>,
    post_render: Vec<StageCallback>,
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

    pub fn push(&mut self, stage: Stage, callback: StageCallback) {
        self.get_mut(stage)
            .push(callback);
    }

    pub fn get(&self, stage: Stage) -> &Vec<StageCallback> {
        match stage {
            Stage::PreUpdate => &self.pre_update, 
            Stage::Update => &self.update,
            Stage::PostUpdate => &self.post_update,
            Stage::PreRender => &self.pre_render,
            Stage::Render => &self.render,
            Stage::PostRender => &self.post_render,
        }
    }

    fn get_mut(&mut self, stage: Stage) -> &mut Vec<StageCallback> {
        match stage {
            Stage::PreUpdate => &mut self.pre_update, 
            Stage::Update => &mut self.update,
            Stage::PostUpdate => &mut self.post_update,
            Stage::PreRender => &mut self.pre_render,
            Stage::Render => &mut self.render,
            Stage::PostRender => &mut self.post_render,
        }
    }
}

#[cfg(test)]
mod stage_tests {
    use test_case::test_case;

    use super::*;
   
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
    
    #[test_case(Stage::PreUpdate)]
    #[test_case(Stage::Update)]
    #[test_case(Stage::PostUpdate)]
    #[test_case(Stage::PreRender)]
    #[test_case(Stage::Render)]
    #[test_case(Stage::PostRender)]
    fn should_add_function_with_correct_callback_group(stage: Stage) {
        let mut stage_callbacks = StageCallbacks::new();

        stage_callbacks.push(stage, |_| {});

        assert_eq!(1, stage_callbacks.get(stage).len(), "The callback was not added to the stage");
    }
}

