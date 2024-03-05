pub use wolf_engine_codegen::DynamicEvent;

use downcast_rs::*;

use std::fmt::Debug;

/// Represents a [`Boxed`](Box) dynamic [`Event`].
pub type EventBox = Box<dyn DynamicEvent>;

/// A dynamically-typed event.
///
/// Events can be downcasted back to their original type using the [`Downcast`] trait.
pub trait DynamicEvent: Downcast + Debug + 'static {}
impl_downcast!(DynamicEvent);

#[cfg(test)]
mod event_tests {
    use test_case::test_case;

    use super::*;

    #[derive(DynamicEvent, Debug)]
    struct TestEvent(&'static str);

    #[test_case(&TestEvent("Hello, World!"))]
    fn should_auto_impl_event(event: &dyn DynamicEvent) {
        if let Some(event) = event.downcast_ref::<TestEvent>() {
            assert_eq!(event.0, "Hello, World!");
        }
    }
}
