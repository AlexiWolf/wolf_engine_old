use crate::State;

pub struct StateMachine {
    stack: Vec<Box<dyn State>>
}

impl StateMachine {
    pub fn new() -> Self {
        Self {
            stack: vec![] 
        }
    }
    
    pub fn push(&mut self, state: Box<dyn State>) {
        self.stack.push(state);
    }

    pub fn pop(&mut self) -> Option<dyn State> {
        None
    }
}

#[cfg(test)]
mod state_machine_tests {
    use super::*;

    #[test]
    fn should_initalize_with_empty_stack() {
        let state_machine = StateMachine::new();
        assert_eq!(
            state_machine.stack.len(),
            0,
            "The state machine was initalized with a state on the stack"
        );
    }

    #[test]
    fn should_push_state_to_stack() {
        let state = fixtures::TestState;
        let mut state_machine = StateMachine::new();
        
        state_machine.push(Box::from(state));

        assert_eq!(
            state_machine.stack.len(),
            1,
            "The state was not pushed to the stack"
        );
    }
    
    #[test]
    fn should_pull_state_off_the_stack() {
        let mut state_machine = StateMachine::new();
        state_machine.push(Box::from(fixtures::TestState));
        
        let state = state_machine.pop();
        
        assert!(state.is_some(), "No state was returned");
    }

    mod fixtures {
        use super::*;

        pub struct TestState; 

        impl State for TestState {}
    }
}
