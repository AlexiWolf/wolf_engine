//! Provides a dynamic main Event-Loop for the engine. 

use crate::dynamic::DynamicEventBox;
use crate::mpsc::{self, MpscEventReceiver, MpscEventSender};
use crate::EventReceiver;
use events::*;

/// Provides a dynamic main Event-Loop for the engine.
///
/// The Event-Loop is a specialized type of [`EventReceiver`], but unlike a typical event receiver,
/// the Event-Loop will continually emit events for as long as it is running, even if there are no 
/// events currently in the queue.  
///
/// When there are no queued events to emit, an [`EventsCleared`] event is returned instead.  When 
/// [`Quit`] is received, the Event-Loop will return [`None`] for all subsequent calls
/// to [`next_event()`](EventReceiver::next_event)
pub struct EventLoop {
    event_receiver: MpscEventReceiver<DynamicEventBox>,
    event_sender: MpscEventSender<DynamicEventBox>,
    has_quit: bool,
}

impl EventLoop {
    pub fn new() -> Self {
        let (event_sender, event_receiver) = mpsc::event_queue();
        Self {
            event_sender,
            event_receiver,
            has_quit: false,
        }
    }

    pub fn event_sender(&self) -> &MpscEventSender<DynamicEventBox> {
        &self.event_sender
    }
}

impl EventReceiver<DynamicEventBox> for EventLoop {
    fn next_event(&mut self) -> Option<DynamicEventBox> {
        if self.has_quit {
            None
        } else {
            match self.event_receiver.next_event() {
                Some(event) => {
                    if event.is::<Quit>() {
                        self.has_quit = true;
                    }
                    Some(event)
                }
                None => Some(Box::from(EventsCleared)),
            }
        }
    }
}

#[cfg(test)]
mod event_loop_tests {
    use super::*;
    use crate::dynamic::DynamicEventSender;
    use ntest::timeout;

    #[test]
    #[timeout(100)]
    fn should_run_and_quit() {
        let mut event_loop = EventLoop::new();
        let mut updates = 0;

        while let Some(event) = event_loop.next_event() {
            if event.is::<EventsCleared>() {
                if updates == 3 {
                    event_loop.event_sender().send_event(Quit).unwrap();
                } else {
                    updates += 1;
                }
            }
        }

        assert!(event_loop.has_quit);
        assert_eq!(updates, 3);
    }

    #[test]
    fn should_emit_events_cleared_when_event_queue_is_empty() {
        let mut event_loop = EventLoop::new();

        assert!(
            event_loop
                .next_event()
                .unwrap()
                .downcast::<EventsCleared>()
                .is_ok(),
            "The event-loop did not emit the expected EventsCleared event."
        );
    }

    #[test]
    #[timeout(100)]
    fn should_not_infinite_loop_when_quit_is_emitted_while_handing_a_quit_event() {
        let mut event_loop = EventLoop::new();
        while let Some(event) = event_loop.next_event() {
            if event.is::<Quit>() || event.is::<EventsCleared>() {
                event_loop.event_sender().send_event(Quit).unwrap();
            }
        }
    }
}
