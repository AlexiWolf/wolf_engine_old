use crate::State;

pub struct StateMachine {
    stack: Vec<Box<dyn State>>
}

impl StateMachine {
    pub fn new_empty() -> Self {
        Self {
            stack: vec![] 
        }
    }
}

#[cfg(test)]
mod state_machine_tests {
    use super::*;

    #[test]
    pub fn should_have_an_empty_constructor() {
        let state_machine = StateMachine::new_empty();
        assert_eq!(
            state_machine.stack.len(),
            0,
            "The state machine was initalized with a state on the stack"
        );
    }
}
