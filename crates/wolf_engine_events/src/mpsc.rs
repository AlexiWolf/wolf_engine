//! Provides a Multi-Producer, Single-Consumer Event-Queue implementation.

use std::sync::mpsc::*;

use crate::*;

/// Creates a new mpsc sender / receiver pair.
pub fn event_queue<E>() -> (MpscEventSender<E>, MpscEventReceiver<E>) {
    let (sender, receiver) = channel();
    let sender = MpscEventSender { inner: sender };
    let receiver = MpscEventReceiver { inner: receiver };
    (sender, receiver)
}

/// Provides the [`EventReceiver`] half of the event queue created by [`event_queue()`].
pub struct MpscEventReceiver<E> {
    inner: Receiver<E>,
}

impl<E: 'static> EventReceiver<E> for MpscEventReceiver<E> {
    fn next_event(&mut self) -> Option<E> {
        self.inner.try_recv().ok()
    }
}

/// Provides the [`EventSender`] half of the event queue created by [`event_queue()`].
pub struct MpscEventSender<E> {
    inner: Sender<E>,
}

impl<E> Clone for MpscEventSender<E> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

// **SAFETY:** This type is backed by `std::sync::mpsc::Sender`, which is `Send` / `Sync`, so, by
// extensions, this type is also safe to be `Send` / `Sync`.
unsafe impl<E> Send for MpscEventSender<E> {}
unsafe impl<E> Sync for MpscEventSender<E> {}

impl<E> EventSender<E> for MpscEventSender<E> {
    fn send_event(&self, event: E) -> Result<(), ReceiverDroppedError> {
        match self.inner.send(event) {
            Ok(_) => Ok(()),
            Err(_) => Err(ReceiverDroppedError),
        }
    }
}

#[cfg(test)]
mod event_queue_tests {
    use std::thread;

    pub use super::*;

    #[test]
    pub fn should_send_and_receive_events() {
        let (event_sender, mut event_queue) = event_queue();

        event_sender.send_event(0).unwrap();

        assert_eq!(event_queue.next_event().expect("No event in the queue"), 0);
    }

    #[test]
    pub fn should_send_events_and_receive_events_across_threads() {
        let (event_sender, mut event_queue) = event_queue();

        event_sender.send_event(0).unwrap();
        let thread_sender = event_sender.clone();
        thread::spawn(move || {
            thread_sender.send_event(1).unwrap();
        })
        .join()
        .unwrap();
        event_sender.send_event(2).unwrap();

        assert_eq!(event_queue.next_event().expect("No event in the queue."), 0);
        assert_eq!(event_queue.next_event().expect("No event in the queue."), 1);
        assert_eq!(event_queue.next_event().expect("No event in the queue."), 2);
    }

    #[test]
    pub fn should_flush_empty_list_if_there_are_no_events() {
        let (_event_sender, mut event_queue) = event_queue::<i32>();

        assert!(event_queue.next_event().is_none());
    }
}
