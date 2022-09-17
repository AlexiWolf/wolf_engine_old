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
    }

    #[test]
    fn should_implement_default() {
        let _stage_callbacks = StageCallbacks::default();
    }
}

