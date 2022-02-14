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
/// let context = ContextBuilder::new()
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
    /// Create the default [ContextBuilder]. 
    pub fn new() -> Self {
        Self::default()
    }

    /// Consumes the `ContextBuilder` and uses it to configure a [Context] object. 
    pub fn build(self) -> Context {
        Context {
            game_loop: GameLoopContext::new(),
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
        Self { event_loop: Some(EventLoop::new()) }
    }

    /// Consumes the [ContextBuilder] and returns a [Context] and an [EventLoop].
    ///
    /// # Panics
    ///
    /// - Will panic if there is no [EventLoop].  
    pub fn build_with_event_loop(self) -> (Context, EventLoop<()>) {
        let context = Context {
            game_loop: GameLoopContext::new(),
        };
        (
            context,
            self.event_loop
                .expect("There is no EventLoop.  Did you mean to use the 'build' method?")
        )
    }
}

impl Default for ContextBuilder {
    fn default() -> Self {
        Self {
            #[cfg(feature = "window")]
            event_loop: None 
        }
    }
}
