use std::sync::Arc;

use crate::events::*;

/// Provides a container for Wolf Engine's user-facing data. 
///
/// Under the hood, Wolf Engine consists of two main parts: The `Context` (You are here!), and the 
/// [`EventLoop`](crate::EventLoop`).  Together, these two parts make up what we refer to as 
/// "the engine."
///
/// The Context owns all engine data, sub-systems, and the link to the Event-Loop through which 
/// all events are sent.  As far as the end-user is concerned, the Context *is* the engine.
/// 
/// # Creating a `Context` 
///
/// A good chunk of Wolf Engine's API depends on the Context.  As such, you will very likely need 
/// to create one *before* trying to use any of the rest of the API.  You can initialize a Context, 
/// along with its associated [`EventLoop`](crate::EventLoop), by calling [`wolf_engine::init()`](crate::init()).
///
/// ```
/// # use wolf_engine_core as wolf_engine;
/// # let data = 0;
/// #
/// let (mut event_loop, mut context) = wolf_engine::init(data);
/// ```
///
/// TODO: Complete the Context docs.
///
/// - [] Make sure to explain custom data.
/// - [] Provide some example code.
/// - [] Revise the intro paragraphs.
///   - I think they're a little awkward, ATM.
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

    pub fn quit(&self) {
        self.event_sender.send_event(Event::Quit).ok();
    }

    pub fn update(&self) {
        self.event_sender.send_event(Event::Update).ok();
    }

    pub fn render(&self) {
        self.event_sender.send_event(Event::Render).ok();
    }
}

impl<D> HasEventSender<Event> for Context<D> {
    fn event_sender(&self) -> Arc<dyn EventSender<Event>> {
        self.event_sender.clone()
    }
}
