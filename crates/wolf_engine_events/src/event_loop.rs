use crate::events::mpsc::{MpscEventReceiver, MpscEventSender};
use crate::events::*;

/// Provides a way to retrieve events from the [`Context`](crate::Context).
///
/// Under the hood, Wolf Engine consists of two main parts: The `EventLoop` (You are here!), and the
/// [`Context`](crate::Context`).  Together, these two parts make up what we refer to as
/// "the engine."
///
/// The Event-Loop is a specialized type of [`EventReceiver`].  Unlike a typical event receiver,
/// the Event-Loop will continually emit events for as long as the engine is running, even if there
/// are no events currently in the queue.  
///
/// When there are no queued events to emit, [`EngineEvent::EventsCleared`] is returned instead, so
/// long as the engine is running.  When [`EngineEvent::Quit`] is received, the event loop will
/// return `None` after the queue is cleared.
///
/// # Examples
///
/// Events are queried using the [`EventReceiver`] API.
///
/// ```
/// # use wolf_engine_core as wolf_engine;
/// # use wolf_engine::prelude::*;
/// #
/// # let (mut event_loop, mut context) = wolf_engine::init().build().unwrap();
/// #
/// while let Some(event) = event_loop.next_event() {
///     match event {
///         // Process events.
/// #       _ => (),
///     }
/// #   break;
/// }
/// ```
pub struct EventLoop {
    event_receiver: MpscEventReceiver<EventBox>,
    event_sender: MpscEventSender<EventBox>,
    has_quit: bool,
}

impl EventLoop {
    pub(crate) fn new() -> Self {
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

    fn handle_event(&mut self, event: &EngineEvent) {
        if *event == EngineEvent::Quit {
            self.has_quit = true;
        }
    }
}

impl EventReceiver<EventBox> for EventLoop {
    fn next_event(&mut self) -> Option<EventBox> {
        if self.has_quit {
            None
        } else {
            match self.event_receiver.next_event() {
                Some(event) => {
                    if let Some(downcast) = event.downcast_ref::<EngineEvent>() {
                        self.handle_event(downcast);
                    }
                    Some(event)
                }
                None => Some(Box::from(EngineEvent::EventsCleared)),
            }
        }
    }
}

#[cfg(test)]
mod event_loop_tests {
    use ntest::timeout;

    use crate::prelude::*;

    #[test]
    #[timeout(100)]
    fn should_run_and_quit() {
        let (mut event_loop, mut context) = crate::init().build().unwrap();
        let mut updates = 0;

        while let Some(event) = event_loop.next_event() {
            if let Ok(event) = event.downcast::<EngineEvent>() {
                process_event(*event, &mut context, &mut updates);
            }
        }

        assert!(event_loop.has_quit);
        assert_eq!(updates, 3);
    }

    fn process_event(event: EngineEvent, context: &mut Context, updates: &mut i32) {
        match event {
            EngineEvent::Quit => (),
            EngineEvent::EventsCleared => {
                if *updates == 3 {
                    context.quit();
                } else {
                    *updates += 1;
                }
            }
        }
    }

    #[test]
    fn should_emit_events_cleared_when_event_queue_is_empty() {
        let (mut event_loop, _context) = crate::init().build().unwrap();

        assert_eq!(
            *event_loop
                .next_event()
                .unwrap()
                .downcast::<EngineEvent>()
                .unwrap(),
            EngineEvent::EventsCleared,
            "The event-loop did not emit the expected EventsCleared event."
        );
    }

    #[test]
    #[timeout(100)]
    fn should_not_infinite_loop_when_quit_is_emitted_while_handing_a_quit_event() {
        let (mut event_loop, context) = crate::init().build().unwrap();
        while let Some(event) = event_loop.next_event() {
            if let Some(engine_event) = event.downcast_ref::<EngineEvent>() {
                match engine_event {
                    EngineEvent::Quit => context.quit(),
                    EngineEvent::EventsCleared => context.quit(),
                }
            }
        }
    }
}
