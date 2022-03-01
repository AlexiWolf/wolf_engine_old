//! Provides access to engine state and tooling.

mod scheduler_context;

pub use scheduler_context::*;

#[cfg(feature = "window")]
use winit::event_loop::EventLoop;

/// Provides storage, and controlled access to global [Engine](crate::Engine) state.
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
}

/// Builds a [Context] object.
#[derive(Default)]
pub struct ContextBuilder {
    #[cfg(feature = "window")]
    event_loop: Option<EventLoop<()>>,
}

impl Default for Context {
    fn default() -> Self {
        ContextBuilder::new().build()
    }
}

impl ContextBuilder {
    /// Create the default [ContextBuilder].
    pub fn new() -> Self {
        Self::default()
    }

    /// Consumes the `ContextBuilder` and uses it to configure a [Context] object.
    pub fn build(self) -> Context {
        Context {
            scheduler: SchedulerContext::new(),
        }
    }
}

#[cfg(feature = "window")]
impl ContextBuilder {
    /// Create an [EventLoop].
    ///
    /// # Panics
    ///
    /// - The [EventLoop] will panic if you attempt to call this function off the main
    ///   thread.  See [EventLoop::new] for more information.
    pub fn with_create_event_loop() -> Self {
        Self {
            event_loop: Some(EventLoop::new()),
        }
    }

    /// Consumes the [ContextBuilder] and returns a [Context] and an [EventLoop].
    ///
    /// # Panics
    ///
    /// - Will panic if there is no [EventLoop].  
    pub fn build_with_event_loop(self) -> (Context, EventLoop<()>) {
        let context = Context {
            scheduler: SchedulerContext::new(),
        };
        (
            context,
            self.event_loop
                .expect("There is no EventLoop.  Did you mean to use the 'build' method?"),
        )
    }
}
