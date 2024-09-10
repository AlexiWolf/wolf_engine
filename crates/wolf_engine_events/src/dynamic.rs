//! Provides dynamic event types.

use downcast_rs::*;

use std::fmt::Debug;

/// A boxed [`Event`].
pub type AnyEvent = Box<dyn Event>;

/// A dynamic event, which can be downcasted back to its original type.
pub trait Event: Downcast + Debug + 'static {}

impl<T> Event for T where T: Debug + 'static {}

impl_downcast!(Event);
