use downcast_rs::*;

use std::fmt::Debug;

pub type AnyEvent = Box<dyn Event>;

pub trait Event: Downcast + Debug + 'static {}
impl_downcast!(Event);
