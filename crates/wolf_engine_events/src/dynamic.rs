//! Provides dynamically-typed events for the engine.

use downcast_rs::*;

use std::fmt::Debug;

use crate::EventSender;
use crate::ReceiverDroppedError;

pub use wolf_engine_codegen::Event;

/// Represents a [`Boxed`](Box) dynamic [`Event`].
pub type EventBox = Box<dyn Event>;

/// An [`EventSender`](crate::EventSender) helper which takes a dynamic [`Event`], and
/// automatically [`Boxes`](Box) it for the caller.
pub trait DynamicEventSender {
    fn send_event<T: Event + 'static>(&self, event: T) -> Result<(), ReceiverDroppedError>
    where
        Self: EventSender<EventBox>,
    {
        EventSender::send_event(self, Box::from(event))
    }
}

impl<T> DynamicEventSender for T where T: EventSender<EventBox> {}

/// A dynamically-typed event.
///
/// Events can be downcasted back to their original type using the [`Downcast`] trait.
pub trait Event: Downcast + Debug + 'static {}
impl_downcast!(Event);

#[cfg(test)]
mod event_tests {
    use test_case::test_case;

    use super::*;

    #[derive(Event, Debug)]
    struct TestEvent(&'static str);

    #[test_case(&TestEvent("Hello, World!"))]
    fn should_auto_impl_event(event: &dyn Event) {
        if let Some(event) = event.downcast_ref::<TestEvent>() {
            assert_eq!(event.0, "Hello, World!");
        }
    }
}
