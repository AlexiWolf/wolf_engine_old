//! Provides access to engine state and tooling.

use std::{fmt::{self, Display, Formatter}, sync::RwLock};

use anymap::AnyMap;

use crate::utils::trust_cell::*;

#[cfg(test)]
use mockall::automock;

/// Indicates a [Subcontext] has already been added to the [Context].
#[derive(Debug)]
pub struct ContextAlreadyExistsError;

impl Display for ContextAlreadyExistsError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "a subcontext of this type already exists: there can be only one \
                    instance per type"
        )
    }
}

/// A marker trait which allows types to be added to the [Context](crate::Context).
#[cfg_attr(test, automock)]
pub trait Subcontext: 'static {}

/// Provides a dynamic storage container for global [Engine](crate::Engine) state.
///
/// This allows for custom [Subcontext] data to be dynamically added, and safely accessed
/// at run-time.  Context utilizes [AnyMap], and [TrustCell] to implement this behavior.
///
/// # Examples
///
/// To create a new context, use [Context::new()];
///
/// ```
/// # use wolf_engine::Context;
/// #
/// let context = Context::new();
/// ```
///
/// Adding a [Subcontext] is done using the [Context::add()] method.
///
/// ```
/// # use wolf_engine::*;
/// #
/// # struct MySubcontext;
/// # impl Subcontext for MySubcontext {}
/// # let my_subcontext = MySubcontext;
/// # let mut context = Context::new();
/// #
/// context.add(my_subcontext);
/// ```
///
/// The [Subcontext] can be accessed again using [Context::borrow()] or
/// [Context::borrow_mut()].
///
/// ```
/// # use wolf_engine::*;
/// #
/// # struct MySubcontext;
/// # impl Subcontext for MySubcontext {}
/// # let subcontext = MySubcontext;
/// # let mut context = Context::new();
/// # context.add(subcontext);
/// #
/// // If you want an immutable reference:
/// if let Some(my_subcontext) = context.borrow::<MySubcontext>() {
///     // Do something with the Subcontext.
/// }
/// # else {
/// #    panic!("No subcontext found");
/// # };
///
/// // If you want a mutable reference:
/// if let Some(my_subcontext_mut) = context.borrow_mut::<MySubcontext>() {
///     // Do something with the Subcontext.
/// }
/// # else {
/// #   panic!("No subcontext found");
/// # };
/// ```
pub struct Context {
    subcontexts: AnyMap,
}

impl Context {
    /// Create a new context with no [Subcontext]s.
    pub fn new() -> Self {
        Self {
            subcontexts: AnyMap::new(),
        }
    }

    /// Add a [Subcontext].
    ///
    /// Only a single instance of a [Subcontext] type may be added.  If the [Subcontext]
    /// cannot be added because another object of the same type is already present, an
    /// [Err] is returned.
    #[allow(clippy::map_entry)]
    pub fn add<T: Subcontext>(&mut self, subcontext: T) -> Result<(), ContextAlreadyExistsError> {
        if self.subcontexts.contains::<RwLock<T>>() {
            Err(ContextAlreadyExistsError)
        } else {
            self.subcontexts.insert(RwLock::new(subcontext));
            Ok(())
        }
    }

    /// Get an immutable reference to a stored [Subcontext].
    ///
    /// Absence of write accesses is checked at run-time.
    ///
    /// # Panics
    ///
    /// This function will panic if there is a mutable reference to the data already in
    /// use.
    pub fn borrow<T: Subcontext>(&self) -> Option<Ref<T>> {
        self.subcontexts
            .get::<TrustCell<T>>()
            .map(|cell| cell.borrow())
    }

    /// Get a mutable reference to a stored [Subcontext].
    ///
    /// Exclusive access is checked at run-time.
    ///
    /// # Panics
    ///
    /// This function will panic if there are any references to the data already in use.
    pub fn borrow_mut<T: Subcontext>(&self) -> Option<RefMut<T>> {
        self.subcontexts
            .get::<TrustCell<T>>()
            .map(|cell| cell.borrow_mut())
    }

    /// Remove a [Subcontext].
    ///
    /// Removing a [Subcontext] is very likely to result in panics or weird behavior from
    /// code depending on it.  Only remove [Subcontext]s if you're absolutely sure they
    /// aren't going to be used again.  Even then, you should only remove types you put
    /// there yourself.
    pub fn remove<T: Subcontext>(&mut self) {
        self.subcontexts.remove::<TrustCell<T>>();
    }

    /// Get the number of [Subcontext] currently stored.
    pub fn len(&self) -> usize {
        self.subcontexts.len()
    }

    /// Returns true if there are no [Subcontext]s currently stored.
    pub fn is_empty(&self) -> bool {
        self.subcontexts.is_empty()
    }
}

impl Default for Context {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod context_tests {
    use log::info;

    use super::*;

    #[test]
    fn should_always_start_with_no_subcontexts() {
        assert!(Context::new().is_empty(), "Context::new() was not empty");
        assert!(
            Context::default().is_empty(),
            "Context::default() was not empty"
        );
    }

    #[test]
    fn should_add_subcontext() {
        let mut context = Context::new();
        let subcontext = MockSubcontext::new();

        context.add(subcontext).expect("failed to add subcontext");

        assert_eq!(context.subcontexts.len(), 1, "The subcontext was not added");
    }

    #[test]
    fn should_allow_only_one_subcontext_of_a_given_type() {
        let mut context = Context::new();
        let subcontext_a = MockSubcontext::new();
        let subcontext_b = MockSubcontext::new();

        let result_a = context.add(subcontext_a);
        let result_b = context.add(subcontext_b);

        assert!(result_a.is_ok(), "adding the first instance should be ok");
        assert!(
            result_b.is_err(),
            "adding the second instance should be an error"
        );
    }

    #[test]
    fn should_remove_subcontext() {
        let mut context = Context::new();
        let subcontext = MockSubcontext::new();
        context.add(subcontext).expect("failed to add subcontext");

        context.remove::<MockSubcontext>();

        assert_eq!(
            context.subcontexts.len(),
            0,
            "the subcontext was not removed"
        );
    }

    #[test]
    fn should_fail_silently_if_removing_nonexistent_subcontext() {
        let mut context = Context::new();

        context.remove::<MockSubcontext>();
    }

    #[test]
    fn should_provide_immutable_access_to_subcontexts() {
        let mut context = Context::new();
        context
            .add(MessageContext::new("Hello, world!"))
            .expect("failed to add subcontext");

        let message_context = context
            .borrow::<MessageContext>()
            .expect("got None instead of the subcontext");

        assert_eq!(message_context.message, "Hello, world!");
    }

    #[test]
    fn should_provide_mutable_access_to_subcontexts() {
        let mut context = Context::new();
        context
            .add(MessageContext::new("Hello, world!"))
            .expect("failed to add subcontext");

        let mut message_context = context
            .borrow_mut::<MessageContext>()
            .expect("got None instead of the subcontext");
        message_context.message = "Goodbye, world!".to_string();

        assert_eq!(message_context.message, "Goodbye, world!");
    }

    #[test]
    fn should_not_fight_the_borrow_checker_on_access_to_subcontexts() {
        let mut context = Context::new();
        context
            .add(MessageContext::new("Hello, world!"))
            .expect("failed to add subcontext");
        context
            .add(PrintContext::new())
            .expect("failed to add subcontext");

        let message_context = context.borrow::<MessageContext>().unwrap();
        let mut print_context = context.borrow_mut::<PrintContext>().unwrap();

        print_context.print(&message_context);

        assert_eq!(print_context.prints, 1);
    }

    struct PrintContext {
        prints: usize,
    }

    impl Subcontext for PrintContext {}

    impl PrintContext {
        pub fn new() -> Self {
            Self { prints: 0 }
        }

        pub fn print(&mut self, message: &MessageContext) {
            info!("{}", message.message);
            self.prints += 1;
        }
    }

    struct MessageContext {
        pub message: String,
    }

    impl MessageContext {
        pub fn new(message: &str) -> Self {
            Self {
                message: message.to_string(),
            }
        }
    }

    impl Subcontext for MessageContext {}
}
