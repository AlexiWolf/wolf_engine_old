pub struct StateMachine;

#[cfg(test)]
mod state_machine_tests {
    use super::*;

    #[test]
    pub fn should_have_an_empty_constructor() {
        let state_machine = StateMachine::new_empty();
        assert_eq!(
            state_machine.stack.size(),
            0,
            "The state machine was initalized with a state on the stack"
        );
    }
}
