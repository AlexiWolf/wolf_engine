use crate::{
    mpsc::{MpscEventReceiver, MpscEventSender},
    EventReceiver,
};

use downcast_rs::*;

use std::fmt::Debug;

pub type AnyEvent = Box<dyn Event>;

pub trait Event: Downcast + Debug + 'static {}
impl_downcast!(Event);

pub struct EventLoop {
    sender: MpscEventSender<AnyEvent>,
    receiver: MpscEventReceiver<AnyEvent>,
}

impl EventLoop {
    pub fn new() -> Self {
        let (sender, receiver) = crate::mpsc::event_queue();
        Self { sender, receiver }
    }

    pub fn event_sender(&self) -> MpscEventSender<AnyEvent> {
        self.sender.to_owned()
    }
}

impl EventReceiver<AnyEvent> for EventLoop {
    fn next_event(&mut self) -> Option<AnyEvent> {
        self.receiver.next_event()
    }
}
