/// The event-receiving half of an Event-Queue.
pub trait EventReceiver<E> {
    /// Returns the next event in the queue.
    fn next_event(&mut self) -> Option<E>;
}

/// The event-sending half of an Event-Queue.
///
/// Event Senders can be freely, and safely cloned, given away, and even sent across threads.
pub trait EventSender<E>: Send + Sync {
    /// Sends an event to the receiver.  
    ///
    /// # Errors
    ///
    /// Returns an error if the receiver has been dropped.
    fn send_event(&self, event: E) -> Result<(), ReceiverDroppedError>;
}

/// An error indicating the [`EventReceiver`] associated with an [`EventSender`] has been dropped,
/// and there is nowhere to send events.
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct ReceiverDroppedError;

impl std::error::Error for ReceiverDroppedError {}

impl std::fmt::Display for ReceiverDroppedError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Failed to send the event, because the receiver was dropped."
        )
    }
}
