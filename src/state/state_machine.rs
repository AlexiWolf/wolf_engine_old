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

    pub fn active(&self) -> Option<&Box<dyn State>> {
        None
    }

    pub fn active_mut(&mut self) -> Option<&mut Box<dyn State>> {
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
    fn should_push_function_to_stack() {
        let state = fixtures::TestState::new(0);
        let mut state_machine = StateMachine::new();
        
        state_machine.push(Box::from(state));

        assert_eq!(
            state_machine.stack.len(),
            1,
            "The state was not pushed to the stack"
        );
    }

    #[test]
    fn should_make_last_state_the_active_state() {
        let mut state_machine = StateMachine::new();

        state_machine.push(Box::from(fixtures::TestState::new(0)));
        assert_eq!(state_machine.active().unwrap().id, 0, "Incorrect active state");

        state_machine.push(Box::from(fixtures::TestState::new(1)));
        assert_eq!(state_machine.active_mut().unwrap().id, 1, "Incorrect active state");
    }
    
    mod fixtures {
        use super::*;

        pub struct TestState {
            pub id: u32
        }

        impl TestState {
            pub fn new(id: u32) -> Self {
                Self {
                    id 
                }
            }
        }

        impl State for TestState {}
    }
}
