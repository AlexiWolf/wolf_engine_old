//! Provides access to engine state and tooling.

mod scheduler_context;

use std::{
    any::{Any, TypeId},
    collections::HashMap,
};

pub use scheduler_context::*;

#[cfg(test)]
use mockall::automock;

#[cfg_attr(test, automock)]
pub trait Subcontext: 'static {}

/// Provides storage and controlled access to global [Engine](crate::Engine) state.
///
/// The context object stores global state for the [Engine](crate::Engine).  Any types
/// that need to work with the [Engine](crate::Engine) can do so through the context
/// object.  Most utility functions will use the context object to do their work.
///
/// The data and state stored by the context object is provided by a number of
/// [Subcontext] objects attached to it.  These [Subcontext]s are added at runtime rather
/// than compile time.
///
/// This works by storing the [Subcontext] as a [Box]ed dyn [Any] object in a map with the
/// [TypeId] of the object is used as the key.  When accessing a stored [Subcontext]
/// object, you must provide the type (`T`) of the object you'd like to access, then
/// the [TypeId] of `T` is used to lookup the corresponding [Subcontext] in the map.  The
/// object is then down-casted back to `T` and returned to the caller
///
/// Because the [TypeId] of the [Subcontext] object is used as the look-up key, there can
/// be only one instance of a specific [Subcontext] type added to the context at a time.
/// Attempting to add another [Subcontext] object with a [TypeId] that's already present
/// in the map will result in a panic.
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
pub struct Context {
    subcontexts: HashMap<TypeId, Box<dyn Any>>,
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
        context.add_subcontext(SchedulerContext::new());
        context
    }

    /// Create an empty context with no [Subcontext]s.
    pub fn empty() -> Self {
        Self {
            subcontexts: HashMap::new(),
        }
    }

    /// Add a [Subcontext] object.
    ///
    /// This function ensures that only a single instance of each [Subcontext] type may
    /// be added.  For example: If you add an instance of `SubcontextA`, then later
    /// attempt to add another instance of `SubcontextA`, this will cause a panic.
    ///
    /// # Panics
    ///
    /// - Will panic if you attempt to add more than one instance of a type.
    #[allow(clippy::map_entry)]
    pub fn add_subcontext<T: Subcontext>(&mut self, subcontext: T) {
        let type_id = TypeId::of::<T>();
        if self.subcontexts.contains_key(&type_id) {
            panic!(
                "a subcontext of this type already exists: there can be only one \
                   instance per type"
            );
        } else {
            self.subcontexts.insert(type_id, Box::from(subcontext));
        }
    }

    /// Access a specific type of [Subcontext] immutably.
    pub fn get_subcontext<T: Subcontext>(&self) -> Option<Box<&T>> {
        let type_id = TypeId::of::<T>();
        if let Some(any) = self.subcontexts.get(&type_id) {
            let subcontext = any.downcast_ref::<T>().expect("failed to downcast");
            Some(Box::from(subcontext))
        } else {
            None
        }
    }

    /// Access a specific type of [Subcontext] mutably.
    pub fn get_subcontext_mut<T: Subcontext>(&mut self) -> Option<Box<&mut T>> {
        let type_id = TypeId::of::<T>();
        if let Some(any) = self.subcontexts.get_mut(&type_id) {
            let subcontext = any.downcast_mut::<T>().expect("failed to downcast");
            Some(Box::from(subcontext))
        } else {
            None
        }
    }

    /// Remove a specific type of [Subcontext].
    ///
    /// You should avoid removing a [Subcontext] unless you're 100% sure no other parts
    /// of the code are depending on it.  Removing a [Subcontext] will likely cause any
    /// code depending on it to panic or otherwise fail.  As a general rule, avoid
    /// removing anything you didn't add yourself.
    pub fn remove_subcontext<T: Subcontext>(&mut self) {
        let type_id = TypeId::of::<T>();
        self.subcontexts.remove(&type_id);
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

        context.add_subcontext(subcontext);

        assert_eq!(context.subcontexts.len(), 1, "The subcontext was not added");
    }

    #[test]
    #[should_panic]
    fn should_allow_only_one_subcontext_of_a_given_type() {
        let mut context = Context::empty();
        let subcontext_a = MockSubcontext::new();
        let subcontext_b = MockSubcontext::new();

        context.add_subcontext(subcontext_a);
        context.add_subcontext(subcontext_b);
    }

    #[test]
    fn should_remove_subcontext() {
        let mut context = Context::empty();
        let subcontext = MockSubcontext::new();
        context.add_subcontext(subcontext);

        context.remove_subcontext::<MockSubcontext>();

        assert_eq!(
            context.subcontexts.len(),
            0,
            "the subcontext was not removed"
        );
    }

    #[test]
    fn should_fail_silently_if_removing_nonexistent_subcontext() {
        let mut context = Context::empty();

        context.remove_subcontext::<MockSubcontext>();
    }

    #[test]
    fn should_provide_immutable_access_to_subcontexts() {
        let mut context = Context::empty();
        context.add_subcontext(MessageContext::new("Hello, world!"));

        let message_context = context
            .get_subcontext::<MessageContext>()
            .expect("got None instead of the subcontext");

        assert_eq!(message_context.message, "Hello, world!");
    }

    #[test]
    fn should_provide_mutable_access_to_subcontexts() {
        let mut context = Context::empty();
        context.add_subcontext(MessageContext::new("Hello, world!"));

        let message_context = context
            .get_subcontext_mut::<MessageContext>()
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
