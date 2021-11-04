//! Provides access to engine state and tooling.

mod game_loop_context;

pub use game_loop_context::*;

/// Provides access to all shared engine state.
///
/// The main context is sort of the gateway to the whole engine, as it stores all of the data for
/// the current running game and engine instance.  You will also access all of the engine tooling
/// through the context object.
///
/// # Examples
///
/// You generally should not build the `Context` object yourself.  Instead, use the
/// [ContextBuilder].
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
pub struct ContextBuilder;

impl ContextBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    /// Consumes the `ContextBuilder` and returns the built [Context] object.
    pub fn build(self) -> Context {
        Context {
            game_loop: GameLoopContext::new(),
        }
    }
}

impl Default for ContextBuilder {
    fn default() -> Self {
        Self 
    }
}
