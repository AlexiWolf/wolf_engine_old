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
/// let event_sender = engine.event_sender();
/// event_sender.send_event(Event::Update);
/// event_sender.send_event(Event::Render);
/// event_sender.send_event(Event::Quit);
///
/// // Use EngineControl methods.
/// engine.update();
/// engine.render();
/// engine.quit();
/// ```
pub trait EngineControls {
    /// Shutdown the engine.
    fn quit(&self);

    /// Check if the engine has quit.
    ///
    /// Returns `true` if the [`Engine`](crate::Engine) has shut down.  Typically after
    /// [`EngineControls::quit()`] has been called, but the [`Engine`](crate::Engine) may shut down
    /// on through other means too.  
    ///
    /// This function may not return `true` immediately after [`EngineControls::quit()`] because
    /// the [`Engine`](crate::Engine) may need time to process the request.
    fn has_quit(&self) -> bool;

    /// Update the engine.
    fn update(&self);

    /// Render a frame to the screen.
    fn render(&self);
}
