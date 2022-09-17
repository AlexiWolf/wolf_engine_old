use crate::Context;

pub type StageCallback = fn(&mut Context);

pub enum Stage {

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

    pub fn push(&mut self, stage: Stage) {}

    pub fn len(&self, stage: Stage) {}
}

#[cfg(test)]
mod stage_tests {
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

    fn should_add_function_with_correct_callback_group(stage: Stage) -> usize {
        let test_callback = |_| {}; 
        let mut stage_callbacks = StageCallbacks::new();
        
        stage_callbacks.push(stage, test_callback);

        stage_callbacks.len(stage)
    }
}

