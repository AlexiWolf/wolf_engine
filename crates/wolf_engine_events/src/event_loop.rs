//! The [EventLoop] trait, and associated types.

use crate::mpsc::MpscEventSender;

/// An event-driven main-loop.
pub trait EventLoop<E> {
    /// Get an event-sender which can send events to the event-loop.
    fn event_sender(&self) -> MpscEventSender<E>;

    /// Run the main-loop.
    fn run<F: FnMut(E)>(self, event_handler: F);
}
