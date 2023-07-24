use std::sync::Arc;

use crate::events::*;
use crate::prelude::*;

/// Provides safe, and controlled access to the user-facing parts of the [Engine].
///
/// The `Context` is a sort of proxy, providing access to specific parts of the [`Engine`].  It
/// holds ownership of user-facing engine data, such as subsystems, and allows for custom data to
/// be used.  The `Context` also provides an [`EventSender`] to send events back to the [`Engine`],
/// provides user-facing controls for the [`Engine`], and acts as a neat package which can be
/// passed to subsystem functions, game functions, ext.
///
/// Why not just use the [`Engine`] directly?
///
/// The [`Engine`] has some components which are not intended to be made accessible to the game.
/// For example, the [`EventQueue`], which uses a Multi-Producer Single-Consumer (`mpsc`) model by
/// default, could not be used by a game without breaking the [`Engine`].  In some cases, it
/// may be impractical, or impossible, either due to borrowing rules, or general complexity of the
/// [`Engine`] type, to borrow the whole [`Engine`].
pub struct Context<D> {
    /// The user-facing engine data.  Normally things like subsystems.
    pub data: D,
    event_sender: Arc<dyn EventSender<Event>>,
}

impl<D> Context<D> {
    /// Create a new `Context` from the provided [`EventQueue`] and data.
    pub fn new(event_queue: &dyn EventQueue<Event>, data: D) -> Self {
        Self {
            data,
            event_sender: event_queue.event_sender(),
        }
    }
}

impl<D> EngineControls for Context<D> {
    fn quit(&self) {
        self.event_sender.send_event(Event::Quit).ok();
    }

    fn update(&self) {
        self.event_sender.send_event(Event::Update).ok();
    }

    fn render(&self) {
        self.event_sender.send_event(Event::Render).ok();
    }
}

impl<D> HasEventSender<Event> for Context<D> {
    fn event_sender(&self) -> Arc<dyn EventSender<Event>> {
        self.event_sender.clone()
    }
}
