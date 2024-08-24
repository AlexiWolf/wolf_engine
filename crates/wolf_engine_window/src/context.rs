use uuid::Uuid;
use wolf_engine_events::{mpsc::MpscEventSender, EventSender};

use crate::{event::BackendEvent, WindowSettings};

#[derive(Clone)]
/// A link to the window system.
pub struct WindowContext {
    event_sender: MpscEventSender<BackendEvent>,
}

impl WindowContext {
    pub(crate) fn new(event_sender: MpscEventSender<BackendEvent>) -> Self {
        Self { event_sender }
    }

    /// Create a new [`Window`](crate::Window).
    pub fn create_window(&self, window_settings: WindowSettings) -> Uuid {
        let uuid = Uuid::new_v4();
        self.event_sender
            .send_event(BackendEvent::CreateWindow(uuid, window_settings))
            .unwrap();
        uuid
    }

    /// Stops the event loop.
    pub fn exit(&self) {
        self.event_sender.send_event(BackendEvent::Exit).unwrap();
    }

    pub(crate) fn event_sender(&self) -> MpscEventSender<BackendEvent> {
        self.event_sender.clone()
    }
}
