/// Provides simplified, and more convenient methods for common operations.
///
/// # Example
///
/// ```
/// # use wolf_engine_core::prelude::*;
/// #
/// # let (engine, context) = Engine::new(());
/// #
/// // Instead of using the EventLoop directly...
/// let event_sender = context.event_sender();
/// event_sender.send_event(Event::Update);
/// event_sender.send_event(Event::Render);
/// event_sender.send_event(Event::Quit);
///
/// // Use EngineControl methods.
/// context.update();
/// context.render();
/// context.quit();
/// ```
pub trait EngineControls {
    /// Shutdown the engine.
    fn quit(&self);

    /// Update the engine.
    fn update(&self);

    /// Render a frame to the screen.
    fn render(&self);
}
