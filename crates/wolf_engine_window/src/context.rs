use uuid::Uuid;
use wolf_engine_events::mpsc::MpscEventSender;

use crate::WindowSettings;

pub(crate) enum ContextEvent {
    CreateWindow(Uuid, WindowSettings),
}

#[derive(Clone)]
/// A link to the window system.
pub struct WindowContext {
    event_sender: MpscEventSender<ContextEvent>,
}

impl WindowContext {
    pub(crate) fn new(event_sender: MpscEventSender<ContextEvent>) -> Self {
        Self { event_sender }
    }

    /// Create a new [`Window`].
    pub fn create_window(&self, window_settings: WindowSettings) -> Uuid {
        let uuid = Uuid::new_v4();
        self.event_sender
            .send_event(ContextEvent::CreateWindow(uuid.clone(), window_settings))
            .unwrap();
        uuid
    }

    /// Stops the event loop.
    pub fn exit(&self) {
        todo!()
    }
}
