use std::fmt::Display;

use crate::{Context, RenderResult, State, UpdateResult};

/// Provides a system for managing and running many [State] objects.
///
/// The State Machine stores a set of [State] objects in a stack, and runs them by
/// delegating function calls down to the states on the stack.  
///
/// state below it is considered "inactive."
///
/// # Active States
///
/// A state is designated as "active" when it's on the top of the stack.  Active states
/// have the following properties:
///
/// - Receive calls from the functions defined by the [State] trait.
/// - Can control the state machine by returning a [Transition] to it.
///
/// # Inactive States
///
/// A state is designated as "inactive" when it's not on the top of the stack.  Inactive
/// states have the following properties:
///
/// - Only receive calls from "background" functions.
///
/// # Examples
///
/// Running the active state:
///
/// ```
/// # use wolf_engine::{StateMachine, State, ContextBuilder};
/// #
/// # let mut context = ContextBuilder::new()
/// #    .build();
/// #
/// let mut state_machine = StateMachine::new();
///
/// loop {
///     state_machine.update(&mut context);
///     state_machine.render(&mut context);
///     # break;
/// }
/// ```
pub struct StateMachine {
    stack: Vec<Box<dyn State>>,
}

impl StateMachine {
    pub fn new() -> Self {
        Self { stack: vec![] }
    }

    pub fn push(&mut self, state: Box<dyn State>) {
        self.stack.push(state);
    }

    pub fn pop(&mut self) -> Option<Box<dyn State>> {
        self.stack.pop()
    }

    pub fn is_empty(&self) -> bool {
        self.stack.is_empty()
    }

    pub fn active_mut(&mut self) -> Option<&mut Box<dyn State>> {
        self.stack.last_mut()
    }
}

impl State for StateMachine {
    fn update(&mut self, context: &mut Context) -> UpdateResult {
        if let Some(state) = self.active_mut() {
            state.update(context);
        }
    }

    fn render(&mut self, context: &mut Context) -> RenderResult {
        if let Some(state) = self.active_mut() {
            state.render(context);
        }
    }
}

impl Display for StateMachine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "State Machine: {} states", self.stack.len())
    }
}

#[cfg(test)]
mod state_machine_tests {
    use crate::{ContextBuilder, Transition};

    use super::*;

    #[test]
    fn should_initialize_with_empty_stack() {
        let state_machine = StateMachine::new();
        assert_eq!(
            state_machine.stack.len(),
            0,
            "The state machine was initialized with a state on the stack"
        );
    }

    #[test]
    fn should_push_state_to_stack() {
        let state = fixtures::TestState::new("default");
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
        state_machine.push(Box::from(fixtures::TestState::new("default")));

        let state = state_machine.pop();

        assert!(state.is_some(), "No state was returned");
    }

    #[test]
    fn should_be_empty_if_there_are_no_states_on_the_stack() {
        let state_machine = StateMachine::new();
        assert!(state_machine.is_empty());
    }

    #[test]
    fn should_not_be_empty_if_there_are_states_on_the_stack() {
        let mut state_machine = StateMachine::new();
        state_machine.push(Box::from(fixtures::TestState::new("default")));

        assert!(!state_machine.is_empty());
    }

    #[test]
    fn should_have_active_state_accessor() {
        let mut state_machine = StateMachine::new();
        state_machine.push(Box::from(fixtures::TestState::new("default")));

        let state = state_machine.active_mut();

        assert!(state.is_some(), "The active state was None");
    }

    #[test]
    fn should_call_state_update() {
        let mut context = ContextBuilder::new().build();
        let mut state_machine = StateMachine::new();
        state_machine.push(Box::from(fixtures::CallTestState::new()));

        for _ in 0..3 {
            state_machine.update(&mut context);
            state_machine.render(&mut context);
        }
        let state = state_machine.pop().unwrap();

        assert_eq!(
            format!("{}", state),
            "3, 3",
            "The state did not have the correct number of calls"
        );
    }

    #[test]
    fn should_handle_pop_transition() {
        let mut context = ContextBuilder::new().build();
        let mut state_machine = StateMachine::new(); 
        state_machine.push(Box::from(fixtures::TransitionTestState::new(Transition::Pop)));
        
        state_machine.update(&mut context);
        
        assert!(state_machine.is_empty(), "The state machine should be empty.");
    }

    mod fixtures {

        use super::*;

        pub struct TestState {
            pub message: String,
        }

        impl TestState {
            pub fn new(message: &str) -> Self {
                Self {
                    message: message.to_string(),
                }
            }
        }

        impl State for TestState {
            fn update(&mut self, _context: &mut Context) -> UpdateResult {
                ()
            }

            fn render(&mut self, _context: &mut Context) -> RenderResult {
                ()
            }
        }

        impl Display for TestState {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.message)
            }
        }

        pub struct CallTestState {
            pub updates: u32,
            pub renders: u32,
        }

        impl CallTestState {
            pub fn new() -> Self {
                Self {
                    updates: 0,
                    renders: 0,
                }
            }
        }

        impl State for CallTestState {
            fn update(&mut self, _context: &mut Context) -> UpdateResult {
                self.updates += 1;
            }

            fn render(&mut self, _context: &mut Context) -> RenderResult {
                self.renders += 1;
            }
        }

        impl Display for CallTestState {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}, {}", self.updates, self.renders)
            }
        }
    
        pub struct TransitionTestState {
            transition: Transition
        }

        impl TransitionTestState {
            pub fn new(transition: Transition) -> Self {
                Self {
                    transition
                }
            }
        }

        impl State for TransitionTestState {

            fn update(&mut self, _context: &mut Context) -> UpdateResult {}

            fn render(&mut self, _context: &mut Context) -> RenderResult {}
        }


        impl Display for TransitionTestState {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "")
            }
        }
    }
}
