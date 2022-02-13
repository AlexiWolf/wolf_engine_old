//! Provides access to engine state and tooling.

mod game_loop_context;

pub use game_loop_context::*;

#[cfg(feature = "window")]
use winit::event_loop::EventLoop;

/// Provides a central hub through which to access all other contexts.
///
/// This is the main context.  It may be helpful to think of it as the "gateway" to the whole engine
/// because it owns all of the contexts active on the engine.  In many cases you will access
/// specific contexts through the main context, but sometimes the main context will have helper
/// functions for common tasks.
///
/// # Examples
///
/// Use the [ContextBuilder] to build a new `Context`.
///
/// ```
/// # use wolf_engine::ContextBuilder;
/// #
/// let (context, event_loop) = ContextBuilder::new()
///     // Insert additional settings here.    
///     .build();
/// ```
pub struct Context {
    pub game_loop: GameLoopContext,
}

/// Builds a [Context] object.
pub struct ContextBuilder {
    #[cfg(feature = "window")]
    event_loop: Option<EventLoop<()>>,
}

impl ContextBuilder {
    /// Use the default [ContextBuilder]
    ///
    /// The [ContextBuilder] can only be initialized on the main thread.  This limitation
    /// comes from the [EventLoop], which is automatically initialized for you by this
    /// method.
    ///
    /// # Panics
    ///
    /// - The [EventLoop] will panic if initialized outside the main thread.
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a [ContextBuilder] that does not have an [EventLoop].  
    ///
    /// This method is really only useful for situations where you need to get around the
    /// "initialize on main thread only" limitation of the [EventLoop].  In almost all
    /// cases, you should probably be using the `new()` or `default()` methods instead.
    #[cfg(feature = "window")]
    fn without_event_loop() -> Self {
        Self { event_loop: None }
    }

    /// Consumes the [ContextBuilder] and returns a [Context] and an [EventLoop].
    ///
    /// # Panics
    ///
    /// - Will panic if there is no [EventLoop].  
    ///
    /// This happens if you create the [ContextBuilder] with
    /// `ContextBuilder::without_event_loop()`.  Use `ContextBuilder::new()` instead.
    #[cfg(feature = "window")]
    pub fn build(self) -> (Context, EventLoop<()>) {
        let context = self.make_context();
        (
            context,
            self.event_loop
                .expect("There is no EventLoop! Did you mean to use ContextBuilder::new()?"),
        )
    }

    /// Consumes the `ContextBuilder` and returns only a [Context] object.
    ///
    /// This method is really only useful for situations where you need to get around the
    /// "initialize on main thread only" limitation of the [EventLoop].  In almost all
    /// cases, you should probably be using the `build` method instead.
    pub fn build(self) -> Context {
        Context {
            game_loop: GameLoopContext::new(),
        }
    }
}

impl Default for ContextBuilder {
    fn default() -> Self {
        Self {
            #[cfg(feature = "window")]
            event_loop: Some(EventLoop::new()),
        }
    }
}
