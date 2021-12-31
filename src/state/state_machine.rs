use std::fmt::Display;

use crate::{Context, RenderResult, State, UpdateResult, Transition};

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
            if let Some(transition) = state.update(context) {
                match transition {
                    Transition::Pop => {self.pop();},
                    Transition::ToState(state) => self.push(state), 
                }
            }
        }
        None
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
    use crate::{ContextBuilder, Transition, MockState};

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
        let state = MockState::new();
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
        state_machine.push(Box::from(MockState::new()));

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
        state_machine.push(Box::from(MockState::new()));

        assert!(!state_machine.is_empty());
    }

    #[test]
    fn should_have_active_state_accessor() {
        let mut state_machine = StateMachine::new();
        state_machine.push(Box::from(MockState::new()));

        let state = state_machine.active_mut();

        assert!(state.is_some(), "The active state was None");
    }

    #[test]
    fn should_call_state_update() {
        let mut context = ContextBuilder::new().build();
        let mut state_machine = StateMachine::new();
        let mut state = MockState::new();
        state.expect_update()
            .times(3)
            .returning(|_| None);
        state_machine.push(Box::from(state));

        for _ in 0..3 {
            state_machine.update(&mut context);
        }
    }

    #[test]
    fn should_handle_pop_transition() {
        let mut context = ContextBuilder::new().build();
        let mut state_machine = StateMachine::new(); 
        let mut state = MockState::new();
        state.expect_update()
            .times(..)
            .returning(|_| Some(Transition::Pop));
        state_machine.push(Box::from(state));
        
        state_machine.update(&mut context);
        
        assert!(state_machine.is_empty(), "The state machine should be empty.");
    }

    #[test]
    fn should_handle_to_state_transition() {
        let mut context = ContextBuilder::new().build();
        let mut state_machine = StateMachine::new();
        let mut transition_to_state = MockState::new();
        let mut no_transition = MockState::new();
        no_transition.expect_update()
            .times(1)
            .returning(|_| None);
        transition_to_state.expect_update()
            .times(1)
            .return_once(move |_| Some(Transition::ToState(Box::from(no_transition))));
        state_machine.push(Box::from(transition_to_state));

        for _ in 0..2 {
            state_machine.update(&mut context);
        }
    }

    #[test]
    fn should_handle_quit_transition() {
        let context = ContextBuilder::new().build();
        let mut state_machine = StateMachine::new();
        let mut state_a = MockState::new();
        let mut state_b = MockState::new();
        let mut quit_state = MockState::new();
        quit_state.expect_update()
            .times(1)
            .returning(|_| Some(Transition::Quit));
        state_machine.push(Box::from(state_a));
        state_machine.push(Box::from(state_b));
        state_machine.push(Box::from(quit_state));

        state_machine.update(&mut context);
    }
}
