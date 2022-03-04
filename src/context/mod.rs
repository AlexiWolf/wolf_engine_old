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
        self.subcontexts.insert(type_id, Box::from(subcontext));
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
}
