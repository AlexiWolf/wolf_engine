//! Provides an event system for Wolf Engine.
//!
//! # Examples
//!
//! This module provides a [FIFO](https://en.wikipedia.org/wiki/FIFO_(computing_and_electronics))
//! (First-in, First-out) event system based on the sender / receiver / message channel found in
//! [std::sync::mpsc].
//!
//! ## Create an Event Queue
//!
//! ```
//! # use wolf_engine_events::*;
//! # enum EventType { Event };
//! #
//! let (event_sender, event_receiver) = mpsc::event_queue();
//! #
//! # event_sender.send_event(123);
//! ```
//!
//! ## Handling Events
//!
//! An [`EventReceiver`] will collect incoming events, and store them until they are ready to be
//! processed.  The order of incoming events is always preserved.
//!
//! Queued events are queried in a loop.  Querying events requires you have mutable access to the
//! Event Queue.
//!
//! ```
//! # use wolf_engine_events::*;
//! # enum EventType { Event };
//! #
//! # let (event_sender, mut event_receiver) = mpsc::event_queue::<EventType>();
//! #
//! while let Some(event) = event_receiver.next_event() {
//!     match event {
//!         EventType::Event => (), // Handle the event.
//!     }
//! }
//! ```
//!
//! ## Sending Events
//!
//! To send an event to an [`EventReceiver`], we use an [`EventSender`].  An event sender is like
//! a tunnel, through which you can send data, and it will pop out on the other side.  
//!
//! ```
//! # use wolf_engine_events::*;
//! # enum EventType { Event };
//! # let (event_sender, event_receiver) = mpsc::event_queue();
//! #
//! event_sender.send_event(EventType::Event);
//! ```
//!
//! ### Cloning, and Transferring Ownership of an `EventSender`
//!
//! Event Senders are extremely useful because they can be freely, and safely cloned, and their
//! ownership moved to other code that needs to send events.  This enables sending events from
//! code that otherwise does not have access to the Event Queue.
//!
//! ```
//! # use wolf_engine_events::*;
//! # #[derive(Clone)]
//! # enum EventType { Event };
//! #
//! # struct SomeOtherType {
//! #     pub event_sender: mpsc::MpscEventSender<EventType>,
//! # }
//! #
//! # impl SomeOtherType {
//! #   fn new(event_sender: mpsc::MpscEventSender<EventType>) -> Self {
//! #       Self { event_sender }
//! #   }
//! # }
//! #
//! # fn some_other_function(event_sender: &mpsc::MpscEventSender<EventType>) {}
//! #
//! # let (event_sender, event_receiver) = mpsc::event_queue();
//! #
//! // The EventSender can be cloned, and freely passed around.
//! let other_type = SomeOtherType::new(event_sender.clone());
//! some_other_function(&event_sender);
//!
//! // The original EventSender is unaffected.
//! event_sender.send_event(EventType::Event);
//! ```
//!
//! ### Sending an `EventSender` to Another Thread
//!
//! Event Senders can be safely sent to other threads.
//!
//! ```
//! # use wolf_engine_events::*;
//! # enum EventType { Event };
//! # let (event_sender, event_receiver) = mpsc::event_queue();
//! #
//! // This EventSender stays on the main thread with the EventReceiver.
//! event_sender.send_event(EventType::Event);
//!
//! // The clone is moved to the other thread.
//! let thread_sender = event_sender.clone();
//! std::thread::spawn(move || {
//!     thread_sender.send_event(EventType::Event);
//! }).join();
//! ```

mod event_queue;
pub use event_queue::*;
pub mod mpsc;
