//! Provides an event system for the engine.
//!
//! Wolf Engine uses a MPSC event system based on the sender / receiver model found in 
//! [std::sync::mpsc::channel] (actually, [MpscEventQueue] is built with the std channel API.)
//! This module provides traits which wrap up the channel-like functionality into a nicer API, so
//! other types, like the [Engine](wolf_engine_core::Engine), and
//! [Context](wolf_engine_core::Context), can have Event Queue functionality.
//!
//! It's important to note, [`EventQueue`] is **not just for events.**  It's actually a generic 
//! message-passing system masquerading as an event system.  It's capable of using any data type 
//! you want to use.  You can absolutely use Event Queues in your game to send any random messages
//! around your game.
//!
//! # Examples
//!
//! All event queues use the same API, so the following examples should work for any type 
//! implementing the [`EventQueue`] traits.  
//!
//! For example: The [`Engine`](wolf_engine_core::Engine) is an event queue, and can be used 
//! just the same.
//!
//! ## Create an Event Queue
//!
//! [`MpscEventQueue`] is the main [`EventQueue`] implementation used by the engine, and it's 
//! simple to set up, so we will use it in our examples.
//!
//! ```
//! # use wolf_engine_core::events::*;
//! # enum EventType { Event };
//! #
//! let event_queue = MpscEventQueue::<EventType>::new();
//! ```
//!
//! As noted above, you can use any custom data you'd like when creating an Event Queue. 
//! For example, numbers!
//! ```
//! # use wolf_engine_core::events::*;
//! #
//! let event_queue = MpscEventQueue::<u32>::new();
//! event_queue.event_sender().send_event(123);
//! ```
//!
//! ## Handling Events
//!
//! An [`EventQueue`] will collect incoming events, until they are ready to be processed.  The
//! order of incoming events is always preserved, and they come out in the same order they came
//! in (FIFO, remember.) 
//!
//! Queued events are queried in a loop.  Querying events requires you have mutable access to the
//! Event Queue, as the Single-Consumer model requires only *one* event consumer.  By requiring
//! mutable access, we can use Rust's type system to enforce this requirement.
//! ## Sending Events
//!
//! When we want to send an event to an [`EventQueue`], we use an [`EventSender`].  An event 
//! sender is like a tunnel, through which you can send data, and it will pop out on the other 
//! side.  
//!
//! ```
//! # use wolf_engine_core::events::*;
//! # enum EventType { Event };
//! # let event_queue = MpscEventQueue::<EventType>::new();
//! #
//! let event_sender = event_queue.event_sender();
//! event_sender.send_event(EventType::Event);
//! ```
//!
//! ### Cloning, and Transferring Ownership of an `EventSender` 
//!
//! The main use-case is enabling events to be sent from code that otherwise does not have access 
//! to the event queue.  Event Senders are extremely useful because they can be freely, and safely 
//! cloned, and their ownership moved to other code that needs to send events.
//!
//!
//! ```
//! # use wolf_engine_core::events::*;
//! # enum EventType { Event };
//! # let event_queue = MpscEventQueue::<EventType>::new();
//! #
//! # struct SomeOtherType {
//! #     pub event_sender: std::sync::Arc<dyn EventSender<EventType>>,
//! # }
//! #
//! # impl SomeOtherType {
//! #   fn new(event_sender: std::sync::Arc<dyn EventSender<EventType>>) -> Self { 
//! #       Self { event_sender } 
//! #   }
//! # }
//! #
//! # fn some_other_function(event_sender: std::sync::Arc<dyn EventSender<EventType>>) {}
//! #
//! let event_sender = event_queue.event_sender();
//!
//! // The EventSender can be cloned, and freely passed around.
//! let other_type = SomeOtherType::new(event_sender.clone()); 
//! some_other_function(event_sender.clone());
//!
//! // The original EventSender is unaffected.
//! event_sender.send_event(EventType::Event);
//! ```
//!
//! ### Sending an `EventSender` to Another Thread 
//!
//! Event Senders can be safely sent across thread boundaries, even when the Event Queue cannot.
//!
//! ```
//! # use wolf_engine_core::events::*;
//! # enum EventType { Event };
//! # let event_queue = MpscEventQueue::<EventType>::new();
//! #
//! // This event sender stays on the main thread.
//! let event_sender = event_queue.event_sender();
//! event_sender.send_event(EventType::Event);
//!
//! // The clone is sent to another thread.
//! let thread_sender = event_sender.clone();
//! std::thread::spawn(move || {
//!     thread_sender.send_event(EventType::Event);
//! }).join();
//! ```
//!
//! ### Sending Events Directly to the `EventQueue`
//!
//! Some [`EventQueue`] implementations may, themselves, also implement [`EventSender`], to allow
//! events to be sent directly.  Assuming you have access to the EventQueue, that is.
//!
//! ```
//! # use wolf_engine_core::events::*;
//! # enum EventType { Event };
//! # let real_event_queue = MpscEventQueue::<EventType>::new();
//! # // Yes, we're faking it.  
//! # // I'm *far* to lazy to init a real example, and it's no different to the reader, so 
//! # // it'll just be our little secret.
//! # let event_queue = real_event_queue.event_sender(); // >;3 
//! #
//! event_queue.send_event(EventType::Event);
//! ```
//!

mod event_queue;
pub use event_queue::*;
mod mpsc_event_queue;
pub use mpsc_event_queue::*;
mod engine_events;
pub use engine_events::*;
