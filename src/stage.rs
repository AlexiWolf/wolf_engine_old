use crate::Context;

pub type StageCallback = fn(&mut Context);

#[derive(Default)]
pub struct StageCallbacks {}

impl StageCallbacks {
    pub fn new() -> Self {
        Self {}
    }
}

#[cfg(test)]
mod stage_tests {
    use super::*;
   
    #[test]
    fn should_create_empty_stage_callbacks() {
        let stage_callbacks = StageCallbacks::new();

        assert_eq!(stage_callbacks.pre_update_callbacks.len(), 0);
        assert_eq!(stage_callbacks.update_callbacks.len(), 0);
        assert_eq!(stage_callbacks.post_update_callbacks.len(), 0);
        assert_eq!(stage_callbacks.pre_render_callbacks.len(), 0);
        assert_eq!(stage_callbacks.render_callbacks.len(), 0);
        assert_eq!(stage_callbacks.post_render_callbacks.len(), 0);
    }

    #[test]
    fn should_implement_default() {
        let _stage_callbacks = StageCallbacks::default();
    }
}

