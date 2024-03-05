use crate::dynamic::{DynamicEvent, DynamicEventBox};
use crate::{EventSender, ReceiverDroppedError};

/// An [`EventSender`](crate::EventSender) helper which takes a [`DynamicEvent`], and
/// automatically [`Boxes`](Box) it for the caller.
pub trait DynamicEventSender {
    fn send_event<T: DynamicEvent + 'static>(&self, event: T) -> Result<(), ReceiverDroppedError>
    where
        Self: EventSender<DynamicEventBox>,
    {
        EventSender::send_event(self, Box::from(event))
    }
}

impl<T> DynamicEventSender for T where T: EventSender<DynamicEventBox> {}
