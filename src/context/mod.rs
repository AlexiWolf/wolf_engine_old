//! Provides access to engine state and tooling.

mod scheduler_context;

use std::{collections::HashMap, any::{TypeId, Any}};

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
/// # Examples
///
/// Use the [ContextBuilder] to build a new context object.
///
/// ```
/// # use wolf_engine::ContextBuilder;
/// #
/// let context = ContextBuilder::new()
///     // Insert additional settings here.    
///     .build();
/// ```
pub struct Context {
    pub scheduler: SchedulerContext,
    subcontexts: HashMap<TypeId, Box<dyn Any>>,
}

impl Context {
    pub fn add_subcontext<T: Subcontext>(&mut self, subcontext: T) {
        let type_id = TypeId::of::<T>();
        if self.subcontexts.contains_key(&type_id) {
            panic!("a subcontext of this type already exists: there can be only one \
                   instance per type" );
        } else {
            self.subcontexts.insert(type_id, Box::from(subcontext));
        }
    }

    pub fn get_subcontext<T: Subcontext>(&mut self) -> Option<Box<&T>> {
        let type_id = TypeId::of::<T>();
        if let Some(any) = self.subcontexts.get(&type_id) {
            let subcontext = any.downcast_ref::<T>().expect("failed to downcast");
            Some(Box::from(subcontext))
        } else {
            None
        }
    }

    pub fn get_subcontext_mut<T: Subcontext>(&mut self) -> Option<Box<&mut T>> {
        let type_id = TypeId::of::<T>();
        if let Some(any) = self.subcontexts.get_mut(&type_id) {
            let subcontext = any.downcast_mut::<T>().expect("failed to downcast");
            Some(Box::from(subcontext))
        } else {
            None
        }
    }
   
}

impl Default for Context {
    fn default() -> Self {
        ContextBuilder::new().build()
    }
}

/// Builds a [Context] object.
#[derive(Default)]
pub struct ContextBuilder {}

impl ContextBuilder {
    /// Create the default [ContextBuilder].
    pub fn new() -> Self {
        Self::default()
    }

    /// Consumes the `ContextBuilder` and uses it to configure a [Context] object.
    pub fn build(self) -> Context {
        Context {
            scheduler: SchedulerContext::new(),
            subcontexts: HashMap::new(),
        }
    }
}

#[cfg(test)]
mod context_tests {
    use super::*;
    
    #[test]
    fn should_allow_custom_sub_contexts() {
        let mut context = Context::default();
        let subcontext = MockSubcontext::new();

        context.add_subcontext(subcontext);

        assert_eq!(context.subcontexts.len(), 1, "The subcontext was not added");
    }

    #[test]
    #[should_panic]
    fn should_allow_only_one_subcontext_of_a_given_type() {
        let mut context = Context::default();
        let subcontext_a = MockSubcontext::new();
        let subcontext_b = MockSubcontext::new();

        context.add_subcontext(subcontext_a);
        context.add_subcontext(subcontext_b);
    }

    #[test]
    fn should_provide_immutable_access_to_subcontexts() {
        let mut context = Context::default();
        context.add_subcontext(MessageContext::new("Hello, world!"));

        let message_context = context.get_subcontext::<MessageContext>()
            .expect("got None instead of the subcontext");
        
        assert_eq!(message_context.message, "Hello, world!");
    }

    #[test]
    fn should_provide_mutable_access_to_subcontexts() {
        let mut context = Context::default();
        context.add_subcontext(MessageContext::new("Hello, world!"));

        let message_context = context.get_subcontext_mut::<MessageContext>()
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
