//! Provides dynamic event types.

use downcast_rs::*;

use std::fmt::Debug;

use crate::EventSender;

/// A boxed [`Event`].
pub type AnyEvent = Box<dyn Event>;

/// An [`EventSender`] exstension to make sending dynamic events more convenient.
pub trait AnyEventSender: EventSender<AnyEvent> {
    /// Sends an [`AnyEvent`] to the receiver.
    ///
    /// See [`send_event`](EventSender::send_event()).
    fn send_any_event<E: Event>(&self, event: E) -> Result<(), crate::ReceiverDroppedError> {
        self.send_event(Box::new(event))
    }
}

impl<T> AnyEventSender for T where T: EventSender<AnyEvent> {}

/// A dynamic event, which can be downcasted back to its original type.
pub trait Event: Downcast + Debug + 'static {}

impl<T> Event for T where T: Debug + 'static {}

impl_downcast!(Event);
