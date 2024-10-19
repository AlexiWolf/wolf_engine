use uuid::Uuid;
use wolf_engine_events::{dynamic::AnyEvent, mpsc::MpscEventSender, EventSender};

use crate::{Window, WindowSettings};

#[derive(Clone)]
/// A link to the window system.
pub struct WindowContext {
    event_sender: MpscEventSender<AnyEvent>,
}

impl WindowContext {
    pub fn new(event_sender: MpscEventSender<AnyEvent>) -> (Self, WindowContextEventSender) {
        let context = Self { event_sender };
        let event_sender = WindowContextEventSender::new(context.clone());
        (context, event_sender)
    }

    /// Create a new [`Window`](crate::Window).
    pub fn create_window(&self, window_settings: WindowSettings) -> Window {
        todo!()
    }

    /// Stops the event loop.
    pub fn exit(&self) {
        todo!()
    }

    fn process_event(&self, event: ()) {}
}

pub struct WindowContextEventSender {
    window_context: WindowContext,
}

impl WindowContextEventSender {
    fn new(window_context: WindowContext) -> Self {
        Self { window_context }
    }
}

impl EventSender<()> for WindowContextEventSender {
    fn send_event(&self, event: ()) -> Result<(), wolf_engine_events::ReceiverDroppedError> {
        self.window_context.process_event(event);
        Ok(())
    }
}

#[cfg(test)]
mod window_context_tests {
    use wolf_engine_events::mpsc;

    use super::*;

    #[test]
    fn should_handle_incoming_events() {
        let (event_sender, event_receiver) = mpsc::event_queue();
        let (context, context_event_sender) = WindowContext::new(event_sender.clone());
        let window = context.create_window(WindowSettings::default().with_size((100, 100)));

        context_event_sender
            .send_event(WindowBackendEvent::WindowResized(window.id(), 800, 600))
            .unwrap();
        assert_eq!(window.size(), (800, 600), "The window was not resized");
    }
}
