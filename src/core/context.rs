//! Provides access to engine state and tooling.

use std::fmt::{self, Display, Formatter};

use anymap::AnyMap;

#[cfg(test)]
use mockall::automock;

use crate::contexts::SchedulerContext;

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

///Provides a dynamic storage container for global [Engine](crate::Engine) state.
///
/// The context object essentially provides a dynamic container for [Subcontext] objects.
/// [Subcontext]s store data used by the engine, engine modules, or the game.
/// Specific [Subcontext]s can be dynamically added, and retrieved by type, at runtime,
/// allowing for greatly improved flexibility, as any type implementing the [Subcontext]
/// trait can be used.  An [AnyMap] is used to achieve this behavior.
///
/// # Examples
///
/// To create the default context, use [Context::default()];
///
/// ```
/// # use wolf_engine::Context;
/// #
/// let context = Context::default();
/// ```
///
/// Adding a [Subcontext] is done using the [Context::add_subcontext()] method.
///
/// ```
/// # use wolf_engine::*;
/// #
/// # struct MySubcontext;
/// # impl Subcontext for MySubcontext {}
/// # let my_subcontext = MySubcontext;
/// # let mut context = Context::empty();
/// #
/// context.add(my_subcontext);
/// ```
///
/// The [Subcontext] can be accessed again using [Context::get_subcontext()] or
/// [Context::get_subcontext_mut()].
///
/// ```
/// # use wolf_engine::*;
/// #
/// # struct MySubcontext;
/// # impl Subcontext for MySubcontext {}
/// # let subcontext = MySubcontext;
/// # let mut context = Context::empty();
/// # context.add(subcontext);
/// #
/// // If you want an immutable reference:
/// if let Some(my_subcontext) = context.get::<MySubcontext>() {
///     // Do something with the Subcontext.
/// }
///
/// // If you want a mutable reference:
/// if let Some(my_subcontext_mut) = context.get_mut::<MySubcontext>() {
///     // Do something with the Subcontext.
/// }
///
pub struct Context {
    subcontexts: AnyMap,
}

impl Context {
    /// Create an instance of the default context.
    ///
    /// The default context starts off with a few common [Subcontext]s.  If this is not
    /// desirable, use [Context::empty()].
    ///
    /// The default [Subcontext]s:
    ///
    /// - [SchedulerContext]
    pub fn new() -> Self {
        let mut context = Self::empty();
        context
            .add(SchedulerContext::new())
            .expect("failed to add SchedulerContext");
        context
    }

    /// Create an empty context with no [Subcontext]s.
    pub fn empty() -> Self {
        Self {
            subcontexts: AnyMap::new(),
        }
    }

    /// Add a [Subcontext] object.
    ///
    /// This function ensures that only a single instance of each [Subcontext] type may
    /// be added.  For example: If you add an instance of `SubcontextA`, then later
    /// attempt to add another instance of `SubcontextA`, this will result in an error.
    ///
    /// A result is returned to indicate if the [Subcontext] was successfully added.  An
    /// [Ok] indicates the context was added, and an [Err] indicates there is already an
    /// instance of the type added.
    #[allow(clippy::map_entry)]
    pub fn add<T: Subcontext>(&mut self, subcontext: T) -> Result<(), ContextAlreadyExistsError> {
        if self.subcontexts.contains::<T>() {
            Err(ContextAlreadyExistsError)
        } else {
            self.subcontexts.insert(subcontext);
            Ok(())
        }
    }

    /// Access a specific type of [Subcontext] immutably.
    pub fn get<T: Subcontext>(&self) -> Option<&T> {
        self.subcontexts.get::<T>()
    }

    /// Access a specific type of [Subcontext] mutably.
    pub fn get_mut<T: Subcontext>(&mut self) -> Option<&mut T> {
        self.subcontexts.get_mut::<T>()
    }

    /// Remove a specific type of [Subcontext].
    ///
    /// You should avoid removing a [Subcontext] unless you're 100% sure no other parts
    /// of the code are depending on it.  Removing a [Subcontext] will likely cause any
    /// code depending on it to panic or otherwise fail.  As a general rule, avoid
    /// removing anything you didn't add yourself.
    pub fn remove<T: Subcontext>(&mut self) {
        self.subcontexts.remove::<T>();
    }
}

impl Default for Context {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod context_tests {
    use super::*;

    #[test]
    fn should_add_subcontext() {
        let mut context = Context::empty();
        let subcontext = MockSubcontext::new();

        context.add(subcontext).expect("failed to add subcontext");

        assert_eq!(context.subcontexts.len(), 1, "The subcontext was not added");
    }

    #[test]
    fn should_allow_only_one_subcontext_of_a_given_type() {
        let mut context = Context::empty();
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
        let mut context = Context::empty();
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
        let mut context = Context::empty();

        context.remove::<MockSubcontext>();
    }

    #[test]
    fn should_provide_immutable_access_to_subcontexts() {
        let mut context = Context::empty();
        context
            .add(MessageContext::new("Hello, world!"))
            .expect("failed to add subcontext");

        let message_context = context
            .get::<MessageContext>()
            .expect("got None instead of the subcontext");

        assert_eq!(message_context.message, "Hello, world!");
    }

    #[test]
    fn should_provide_mutable_access_to_subcontexts() {
        let mut context = Context::empty();
        context
            .add(MessageContext::new("Hello, world!"))
            .expect("failed to add subcontext");

        let message_context = context
            .get_mut::<MessageContext>()
            .expect("got None instead of the subcontext");
        message_context.message = "Goodbye, world!".to_string();

        assert_eq!(message_context.message, "Goodbye, world!");
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
