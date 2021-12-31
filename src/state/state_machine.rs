use std::fmt::Display;

use crate::{Context, RenderResult, State, Transition, UpdateResult};

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
    /// Create a new state machine with an empty stack.
    pub fn new() -> Self {
        Self { stack: vec![] }
    }
    
    /// Apply the provided [Transition] to the state machine.
    pub fn do_transition(&mut self, update_result: UpdateResult) {
        if let Some(transition) = update_result {
            match transition {
                Transition::Push(state) => self.push(state),
                Transition::Pop => self.pop_no_return(),
                Transition::CleanPush(state) => self.clean_push(state),
                Transition::Quit => self.clean(),
            }
        }
    }

    pub fn push(&mut self, state: Box<dyn State>) {
        self.stack.push(state);
    }

    pub fn pop_no_return(&mut self) {
        self.pop();
    }

    pub fn pop(&mut self) -> Option<Box<dyn State>> {
        self.stack.pop()
    }

    pub fn clean_push(&mut self, state: Box<dyn State>) {
        self.clean();
        self.push(state);
    }

    pub fn clean(&mut self) {
        while !self.is_empty() {
            self.pop();
        }
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
            let update_result = state.update(context);
            self.do_transition(update_result);
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

impl Default for StateMachine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod state_machine_tests {
    use crate::{ContextBuilder, MockState, Transition};

    use super::*;

    #[test]
    fn should_start_with_empty_stack() {
        let state_machine = StateMachine::new();

        assert_eq!(
            state_machine.stack.len(),
            0,
            "The state machine was initialized with a state on the stack"
        );
    }

    #[test]
    fn should_push_state_on_the_stack() {
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
    fn should_pop_state_off_the_stack() {
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
    fn should_handle_none_transition() {
        let (mut context, mut state_machine) = new_context_and_state_machine();
        let mut state = MockState::new();
        state.expect_update().times(3).returning(|_| None);

        state_machine.push(Box::from(state));
        for _ in 0..3 {
            state_machine.update(&mut context);
        }
    }

    #[test]
    fn should_handle_pop_transition() {
        let (mut context, mut state_machine) = new_context_and_state_machine();
        let mut state = MockState::new();
        state
            .expect_update()
            .times(1)
            .returning(|_| Some(Transition::Pop));

        state_machine.push(Box::from(state));
        state_machine.update(&mut context);

        assert!(
            state_machine.is_empty(),
            "The state machine should be empty."
        );
    }

    #[test]
    fn should_handle_to_state_transition() {
        let (mut context, mut state_machine) = new_context_and_state_machine();
        let mut transition_to_state = MockState::new();
        let mut no_transition = MockState::new();
        no_transition.expect_update().times(1).returning(|_| None);
        transition_to_state
            .expect_update()
            .times(1)
            .return_once(move |_| Some(Transition::Push(Box::from(no_transition))));

        state_machine.push(Box::from(transition_to_state));
        for _ in 0..2 {
            state_machine.update(&mut context);
        }
    }

    #[test]
    fn should_handle_clean_push_transition() {
        let (mut context, mut state_machine) = new_context_and_state_machine();
        let mut no_transition_state = MockState::new();
        no_transition_state
            .expect_update()
            .times(1)
            .returning(|_| None);
        let mut clean_push_state = MockState::new();
        clean_push_state
            .expect_update()
            .times(1)
            .return_once(move |_| Some(Transition::CleanPush(Box::from(no_transition_state))));

        state_machine.push(Box::from(clean_push_state));
        for _ in 0..2 {
            state_machine.update(&mut context);
        }
    }

    #[test]
    fn should_handle_quit_transition() {
        let (mut context, mut state_machine) = new_context_and_state_machine();
        add_placeholder_states(&mut state_machine);
        let mut quit_state = MockState::new();
        quit_state
            .expect_update()
            .times(1)
            .returning(|_| Some(Transition::Quit));

        state_machine.push(Box::from(quit_state));
        state_machine.update(&mut context);

        assert!(
            state_machine.is_empty(),
            "The stack should be empty, but it is not"
        );
    }

    fn add_placeholder_states(state_machine: &mut StateMachine) {
        state_machine.push(Box::from(MockState::new()));
        state_machine.push(Box::from(MockState::new()));
    }

    fn new_context_and_state_machine() -> (Context, StateMachine) {
        let context = ContextBuilder::new().build();
        let state_machine = StateMachine::new();
        (context, state_machine)
    }
}
