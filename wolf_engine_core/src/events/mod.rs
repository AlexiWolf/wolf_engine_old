//! Provides an event system for the engine.
//!
//! This module uses a [FIFO](https://en.wikipedia.org/wiki/FIFO_(computing_and_electronics))
//! (First-in, First-out), MPSC (Multi-Producer, Single-Consumer) event system based on the
//! sender / receiver model found in [std::sync::mpsc::channel] (actually, [MpscEventQueue] is
//! built on the std channel API.) This module provides traits which wrap up the channel-like
//! functionality into a nicer API, so other types, like the [EventLoop](crate::EventLoop),
//! and [Context](crate::Context), can have Event Queue functionality.
//!
//! It's important to note, [`EventQueue`] is **not just for events.**  It's actually a generic
//! message-passing system masquerading as an event system.  It's capable of using any data type
//! you want to use.  You can absolutely use Event Queues in your game to send any random messages
//! around.
//!
//! # Examples
//!
//! All event queues use the same API, so the following examples should work for any type
//! implementing the [`EventQueue`] traits.  
//!
//! ## Create an Event Queue
//!
//! [`MpscEventQueue`] is the main [`EventQueue`] implementation used by the engine, and since
//! it's simple to set up, we will use it in our examples.
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
//!
//! ```
//! # use wolf_engine_core::events::*;
//! #
//! let event_queue = MpscEventQueue::<u32>::new();
//! event_queue.event_sender().send_event(123);
//! ```
//!
//! ## Handling Events
//!
//! An [`EventQueue`] will collect incoming events, and store them until they are ready to be
//! processed.  The order of incoming events is always preserved, and they come out in the same
//! order they came in.  (FIFO, remember?)
//!
//! Queued events are queried in a loop.  Querying events requires you have mutable access to the
//! Event Queue, as the Single-Consumer model can only have *one* event consumer.  By requiring
//! mutable access, we can use Rust's type system better enforce this restriction
//!
//! ```
//! # use wolf_engine_core::events::*;
//! # enum EventType { Event };
//! # let mut event_queue = MpscEventQueue::<EventType>::new();
//! #
//! while let Some(event) = event_queue.next_event() {
//!     match event {
//!         EventType::Event => (), // Handle the event.
//!     }
//! }
//! ```
//!
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
//! event_sender.send_event(EventType::Event); // Event is sent back to the EventQueue.
//! ```
//!
//! ### Cloning, and Transferring Ownership of an `EventSender`
//!
//! Event Senders are extremely useful because they can be freely, and safely cloned, and their
//! ownership moved to other code that needs to send events.  This enables sending events from
//! code that otherwise does not have access to the Event Queue.
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
//! // This EventSender stays on the main thread with the EventQueue.
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
//! events to be sent directly, without needing to crate an Event Sender.  
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

mod event_queue;
pub use event_queue::*;
mod mpsc_event_queue;
pub use mpsc_event_queue::*;
mod engine_events;
pub use engine_events::*;
