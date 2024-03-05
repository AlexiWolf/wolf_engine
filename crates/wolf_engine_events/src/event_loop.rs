use generic_event_queue::EventSender;

use crate::mpsc::{self, MpscEventReceiver, MpscEventSender};
use crate::dynamic::{EventBox, Event};
use crate::EventReceiver;

#[derive(Event, Debug)]
pub struct Quit;

#[derive(Event, Debug)]
pub struct EventsCleared;

pub struct EventLoop {
    event_receiver: MpscEventReceiver<EventBox>,
    event_sender: MpscEventSender<EventBox>,
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

    pub fn event_sender(&self) -> &MpscEventSender<EventBox> {
        &self.event_sender
    }
}

impl EventReceiver<EventBox> for EventLoop {
    fn next_event(&mut self) -> Option<EventBox> {
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
    use ntest::timeout;
    use super::*;

    #[test]
    #[timeout(100)]
    fn should_run_and_quit() {
        let mut event_loop = EventLoop::new();
        let mut updates = 0;

        while let Some(event) = event_loop.next_event() {
            if let Ok(event) = event.downcast::<EventsCleared>() {
                if updates == 3 {
                    event_loop.event_sender().send_event(Box::from(Quit));
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
                event_loop.event_sender().send_event(Box::from(Quit));
            }
        }
    }
}
