use crate::mpsc::MpscEventSender;

pub trait EventLoop<E> {
    fn event_sender(&self) -> MpscEventSender<E>;
    fn run<F: FnMut(E)>(self, event_handler: F);
}
