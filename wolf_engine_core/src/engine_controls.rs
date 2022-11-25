/// Provides simplified, and more convenient methods for common operations.
///
/// # Example
///
/// ```
/// # use wolf_engine_core::prelude::*;
/// #
/// # let engine = Engine::default();
/// #
/// // Instead of using the EventLoop directly...
///
/// engine.send_event(Event::Update);
/// engine.send_event(Event::Render);
/// engine.send_event(Event::Quit);
///
/// // Use EngineControl methods.
///
/// engine.update();
/// engine.render();
/// engine.quit();
/// ```
pub trait EngineControls {
    /// Shutdown the engine.
    fn quit(&self);
    /// Check if the engine has quit.
    ///
    /// Returns `true` if [EngineControls::quit()] has been called, or if the engine has shut down
    /// through other means.
    fn has_quit(&self) -> bool;
    /// Update the engine.
    fn update(&self);
    /// Render a frame to the screen.
    fn render(&self);
}
