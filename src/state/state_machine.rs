use std::fmt::Display;

use crate::{State, UpdateResult, RenderResult, Context};

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
///
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
    fn update(&mut self, _context: &mut Context) -> UpdateResult {
        ()
    }

    fn render(&mut self, _context: &mut Context) -> RenderResult {
        () 
    }
}

impl Display for StateMachine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "State Machine: {} states", self.stack.len())
    }
}

#[cfg(test)]
mod state_machine_tests {
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
    fn hould_push_state_to_stack() {
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
    
    #[test]
    fn should_be_empty_if_there_are_no_states_on_the_stack() {
        let state_machine = StateMachine::new();
        assert!(state_machine.is_empty());
    }

    #[test]
    fn should_not_be_empty_if_there_are_states_on_the_stack() {
        let mut state_machine = StateMachine::new();
        state_machine.push(Box::from(fixtures::TestState));

        assert!(!state_machine.is_empty());
    }


    #[test]
    fn should_have_active_state_accessor() { 
        let mut state_machine = StateMachine::new();
        state_machine.push(Box::from(fixtures::TestState));
        
        let state = state_machine.active_mut();

        assert!(state.is_some(), "The active state was None");
    }

    mod fixtures {

        use super::*;

        pub struct TestState; 

        impl State for TestState {
            fn update(&mut self, _context: &mut Context) -> UpdateResult {
                ()                
            }

            fn render(&mut self, _context: &mut Context) -> RenderResult {
                () 
            }
        }
    }
}
