use std::{fmt::Display, iter::Take, slice::IterMut};

use crate::{Context, OptionalTransition, RenderResult, State, Transition};
/// Provides a stack for storing, managing, and running multiple [State] objects.
///
/// The state stack acts as a common interface through which numerous [State]s can be run
/// and managed.  The state stack is essentially a specialized [State] designed to run
/// other [State] objects.  Attached [State]s are stored on a stack, and the state stack
/// delegates `update` and `render` calls to objects on the stack in a specific order.
///
/// The state stack will update and render all [State]s stored on the stack.  This allows
/// the game to switch modes but continue doing things in the background.  It may help to
/// think about the [State]s as being "layered" on top of each other.
///
/// For example: You may want to implement an inventory screen that pops up on top of the
/// game, but you don't want your game to stop.  Using the stack-based approach allows you
/// to push your inventory to the top of the stack, and the now "active" inventory can
/// consume inputs, while the game continues to run in the background.  
///
/// If a "layered" behavior is not desirable, the `clean_push` [Transition] will pop all
/// states off the stack before pushing the new state.  Clean pushing [State]s makes the
/// state stack feel more like a finite state machine.
///
/// # Active State
///
/// The "active" state is whatever state is currently on the top of the stack.  Active
/// states have the following properties:
///
/// - Only the active state will have it's "foreground" `[update/render]` methods called.
/// - The active state will not have it's `background_[update/render]` methods called.
/// - The active state is the only stat that can send a [Transition] to the state stack.
///
/// Normally, the active state is going to be the "mode" your game is in.
///
/// # Inactive States
///
/// A state is designated as "inactive" when it's not on the top of the stack.  Inactive
/// states have the following properties:
///
/// - Inactive states will only have their `background_[update/render]` methods called.
/// - Inactive states will not have their "foreground" `[update/render]` methods called.
/// - Inactive states cannot send a [Transition] to the state stack.
///
/// # Update / Render Order
///
/// The states are always updated and rendered in bottom-to-top order, with the active
/// state going last.  This allows the top states to display graphics over the states
/// below them.
///
/// # Examples
///
/// Running the state stack:
///
/// ```
/// # use wolf_engine::{StateStack, State, Context};
/// #
/// # let mut context = Context::default();
/// #
/// let mut state_stack = StateStack::new();
///
/// loop {
///     state_stack.update(&mut context);
///     state_stack.render(&mut context);
///     # break;
/// }
/// ```
///
/// See the [examples folder](https://github.com/AlexiWolf/wolf_engine/tree/main/examples)
/// for a more complete example how to use [State]s and the state stack.
pub struct StateStack {
    stack: Vec<Box<dyn State>>,
}

impl StateStack {
    /// Create a new state stack with an empty stack.
    pub fn new() -> Self {
        Self { stack: vec![] }
    }

    /// Is true if there are any [State]s on the stack.
    pub fn is_not_empty(&self) -> bool {
        !self.is_empty()
    }

    /// Is true if there are no [State]s on the stack.
    pub fn is_empty(&self) -> bool {
        self.stack.is_empty()
    }

    /// Get a mutable reference to the active [State].
    pub fn active_mut(&mut self) -> Option<&mut Box<dyn State>> {
        self.stack.last_mut()
    }

    fn do_transition(&mut self, update_result: OptionalTransition, context: &mut Context) {
        if let Some(transition) = update_result {
            match transition {
                Transition::Push(state) => self.push(state, context),
                Transition::Pop => self.pop_no_return(context),
                Transition::CleanPush(state) => self.clean_push(state, context),
                Transition::Quit => self.clear(context),
            }
        }
    }

    /// Push the provided [State] to the top of the stack.
    ///
    /// The state will become the new active state.
    pub fn push(&mut self, mut state: Box<dyn State>, context: &mut Context) {
        self.pause_active_state(context);
        state.setup(context);
        self.stack.push(state);
    }

    fn pause_active_state(&mut self, context: &mut Context) {
        if let Some(active_state) = self.active_mut() {
            active_state.pause(context);
        }
    }

    fn pop_no_return(&mut self, context: &mut Context) {
        self.pop(context);
    }

    fn pop(&mut self, context: &mut Context) -> Option<Box<dyn State>> {
        if let Some(mut state) = self.stack.pop() {
            state.shutdown(context); 
            Some(state)
        } else {
            None
        }
    }

    fn clean_push(&mut self, state: Box<dyn State>, context: &mut Context) {
        self.clear(context);
        self.push(state, context);
    }

    /// Pop all states off the stack.
    pub fn clear(&mut self, context: &mut Context) {
        while !self.is_empty() {
            self.pop(context);
        }
    }

    fn take_background_states(&mut self) -> Take<IterMut<Box<dyn State>>> {
        let stack_size = self.stack.len();
        let background_states = match stack_size {
            1.. => stack_size - 1,
            _ => 0,
        };
        self.stack.iter_mut().take(background_states)
    }
}

impl State for StateStack {
    fn update(&mut self, context: &mut Context) -> OptionalTransition {
        self.take_background_states()
            .for_each(|state| state.background_update(context));
        if let Some(state) = self.active_mut() {
            let update_result = state.update(context);
            self.do_transition(update_result, context);
        }
        None
    }

    fn render(&mut self, context: &mut Context) -> RenderResult {
        self.take_background_states()
            .for_each(|state| state.background_render(context));
        if let Some(state) = self.active_mut() {
            state.render(context);
        }
    }
}

impl Display for StateStack {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "State Machine: {} states", self.stack.len())
    }
}

impl Default for StateStack {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod state_stack_tests {
    use crate::{MockState, Transition};

    use super::*;

    #[test]
    fn should_start_with_empty_stack() {
        let state_stack = StateStack::new();

        assert_eq!(
            state_stack.stack.len(),
            0,
            "The state stack was initialized with a state on the stack"
        );
    }

    #[test]
    fn should_push_state_on_the_stack() {
        let (mut context, mut state_stack) = new_context_and_state_stack();
        let state = new_mock_state_with_setup_expectation();

        state_stack.push(Box::from(state), &mut context);

        assert_eq!(
            state_stack.stack.len(),
            1,
            "The state was not pushed to the stack"
        );
    }

    #[test]
    fn should_pop_state_off_the_stack() {
        let (mut context, mut state_stack) = new_context_and_state_stack();
        let mut state = new_mock_state_with_setup_expectation();
        expect_shutdown(&mut state);

        state_stack.push(Box::from(state), &mut context);
        let state = state_stack.pop(&mut context);

        assert!(state.is_some(), "No state was returned");
    }

    #[test]
    fn should_be_empty_if_there_are_no_states_on_the_stack() {
        let state_stack = StateStack::new();

        assert!(state_stack.is_empty());
    }

    #[test]
    fn should_not_be_empty_if_there_are_states_on_the_stack() {
        let (mut context, mut state_stack) = new_context_and_state_stack();

        state_stack.push(Box::from(new_mock_state_with_setup_expectation()), &mut context);

        assert!(!state_stack.is_empty());
    }

    #[test]
    fn should_have_active_state_accessor() {
        let (mut context, mut state_stack) = new_context_and_state_stack();
        state_stack.push(Box::from(new_mock_state_with_setup_expectation()), &mut context);

        let state = state_stack.active_mut();

        assert!(state.is_some(), "The active state was None");
    }

    #[test]
    fn should_handle_none_transition() {
        let (mut context, mut state_stack) = new_context_and_state_stack();
        let mut state = new_mock_state_with_setup_expectation();
        state.expect_update().times(3).returning(|_| None);

        state_stack.push(Box::from(state), &mut context);
        for _ in 0..3 {
            state_stack.update(&mut context);
        }
    }

    #[test]
    fn should_handle_pop_transition() {
        let (mut context, mut state_stack) = new_context_and_state_stack();
        let mut state = new_mock_state_with_setup_expectation();
        state
            .expect_update()
            .times(1)
            .returning(|_| Some(Transition::Pop));
        expect_shutdown(&mut state);

        state_stack.push(Box::from(state), &mut context);
        state_stack.update(&mut context);

        assert!(state_stack.is_empty(), "The state stack should be empty.");
    }

    #[test]
    fn should_handle_to_state_transition() {
        let (mut context, mut state_stack) = new_context_and_state_stack();
        let mut transition_to_state = new_mock_state_with_setup_expectation();
        let mut no_transition = new_mock_state_with_setup_expectation();
        no_transition.expect_update().times(9).returning(|_| None);
        transition_to_state
            .expect_update()
            .times(1)
            .return_once(move |_| Some(Transition::Push(Box::from(no_transition))));
        transition_to_state
            .expect_background_update()
            .times(9)
            .returning(|_| ());
        expect_pause(&mut transition_to_state);

        state_stack.push(Box::from(transition_to_state), &mut context);
        for _ in 0..10 {
            state_stack.update(&mut context);
        }
    }

    #[test]
    fn should_handle_clean_push_transition() {
        let (mut context, mut state_stack) = new_context_and_state_stack();
        let mut no_transition_state = new_mock_state_with_setup_expectation();
        no_transition_state
            .expect_update()
            .times(1)
            .returning(|_| None);
        let mut clean_push_state = new_mock_state_with_setup_expectation();
        clean_push_state
            .expect_update()
            .times(1)
            .return_once(move |_| Some(Transition::CleanPush(Box::from(no_transition_state))));
        expect_shutdown(&mut clean_push_state);

        state_stack.push(Box::from(clean_push_state), &mut context);
        for _ in 0..2 {
            state_stack.update(&mut context);
        }
    }

    #[test]
    fn should_handle_quit_transition() {
        let (mut context, mut state_stack) = new_context_and_state_stack();
        let mut quit_state = new_mock_state_with_setup_expectation();
        quit_state
            .expect_update()
            .times(1)
            .returning(|_| Some(Transition::Quit));
        expect_shutdown(&mut quit_state);

        state_stack.push(Box::from(quit_state), &mut context);
        state_stack.update(&mut context);

        assert!(
            state_stack.is_empty(),
            "The stack should be empty, but it is not"
        );
    }

    #[test]
    fn should_run_background_update_for_background_states() {
        let (mut context, mut state_stack) = new_context_and_state_stack();
        let mut state_a = new_mock_state_with_setup_expectation();
        state_a
            .expect_background_update()
            .times(10)
            .returning(|_| ());
        expect_pause(&mut state_a);
        let mut state_b = new_mock_state_with_setup_expectation();
        state_b.expect_update().times(10).returning(|_| None);
        state_stack.push(Box::from(state_a), &mut context);
        state_stack.push(Box::from(state_b), &mut context);

        for _ in 0..10 {
            state_stack.update(&mut context);
        }
    }

    #[test]
    fn should_run_background_render_for_background_states() {
        let (mut context, mut state_stack) = new_context_and_state_stack();
        let mut state_a = new_mock_state_with_setup_expectation();
        state_a
            .expect_background_render()
            .times(10)
            .returning(|_| ());
        expect_pause(&mut state_a);
        let mut state_b = new_mock_state_with_setup_expectation();
        state_b.expect_render().times(10).returning(|_| ());
        state_stack.push(Box::from(state_a), &mut context);
        state_stack.push(Box::from(state_b), &mut context);

        for _ in 0..10 {
            state_stack.render(&mut context);
        }
    }

    #[test]
    fn should_run_with_empty_stack() {
        let (mut context, mut state_stack) = new_context_and_state_stack();

        for _ in 0..10 {
            state_stack.update(&mut context);
            state_stack.render(&mut context);
        }
    }

    #[test]
    fn should_run_startup_method_when_state_is_added() {
        let (mut context, mut state_stack) = new_context_and_state_stack();
        let mut state = MockState::new();
        state.expect_setup()
            .times(1)
            .returning(|_| ());

        state_stack.push(Box::from(state), &mut context);
    }

    #[test]
    fn should_run_shutdown_method_when_state_is_removed() {
        let (mut context, mut state_stack) = new_context_and_state_stack();
        let mut state = new_mock_state_with_setup_expectation();
        expect_shutdown(&mut state);
        state_stack.push(Box::from(state), &mut context);

        state_stack.pop(&mut context);
    }

    #[test]
    fn should_run_pause_method_when_state_is_deactivated() {
        let (mut context, mut state_stack) = new_context_and_state_stack();
        let mut state_a = new_mock_state_with_setup_expectation();
        let state_b = new_mock_state_with_setup_expectation();
        expect_pause(&mut state_a);
        
        state_stack.push(Box::from(state_a), &mut context);
        state_stack.push(Box::from(state_b), &mut context);
    }

    #[test]
    fn should_run_resume_method_when_state_is_reactivated() {
        let (mut context, mut state_stack) = new_context_and_state_stack();
        let mut state_a = new_mock_state_with_setup_expectation();
        let mut state_b = new_mock_state_with_setup_expectation();
        expect_pause(&mut state_a);
        expect_shutdown(&mut state_b);
        state_a.expect_resume().times(1).returning(|_| ());

        state_stack.push(Box::from(state_a), &mut context);
        state_stack.push(Box::from(state_b), &mut context);
        state_stack.pop(&mut context);
    }

    fn new_context_and_state_stack() -> (Context, StateStack) {
        let context = Context::new();
        let state_stack = StateStack::new();
        (context, state_stack)
    }

    fn new_mock_state_with_setup_expectation() -> MockState {
        let mut state = MockState::new();
        expect_startup(&mut state);
        state
    }

    fn expect_startup(state: &mut MockState) {
        state.expect_setup().times(0..2).returning(|_| ());
    }

    fn expect_shutdown(state: &mut MockState) {
        state.expect_shutdown().times(1).returning(|_| ());
    }

    fn expect_pause(state: &mut MockState) {
        state.expect_pause().times(1).returning(|_| ());
    }
}
