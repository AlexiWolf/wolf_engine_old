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
/// For example, the [`EventLoop`], which uses a Multi-Producer Single-Consumer (`mpsc`) model by
/// default, could not be used by a game without breaking the [`Engine`].  In some cases, it
/// may be impractical, or impossible, either due to borrowing rules, or general complexity of the
/// [`Engine`] type, to borrow the whole [`Engine`].
pub struct Context<D> {
    /// The user-facing engine data.  Normally things like subsystems.
    pub data: D,
    event_sender: Arc<dyn EventSenderProxy<Event>>,
    has_quit: bool,
}

impl<D> Context<D> {
    /// Create a new `Context` from the provided [`EventLoop`] and data.
    pub fn new(event_loop: &dyn EventLoop<Event>, data: D) -> Self {
        Self {
            data,
            event_sender: event_loop.sender(),
            has_quit: false,
        }
    }

    /// Set the `has_quit` flag.
    ///
    /// In most cases, this function should only be called by the [`Engine`].
    pub(crate) fn set_has_quit(&mut self, has_quit: bool) {
        self.has_quit = has_quit;
    }
}

impl<D> EngineControls for Context<D> {
    fn quit(&self) {
        self.event_sender.send_event(Event::Quit).ok();
    }

    fn has_quit(&self) -> bool {
        self.has_quit
    }

    fn update(&self) {
        self.event_sender.send_event(Event::Update).ok();
    }

    fn render(&self) {
        self.event_sender.send_event(Event::Render).ok();
    }
}
