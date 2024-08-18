use uuid::Uuid;
use wolf_engine_events::mpsc::MpscEventSender;

use crate::WindowSettings;

pub(crate) enum ContextEvent {
    CreateWindow(Uuid),
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
        todo!()
    }

    /// Stops the event loop.
    pub fn exit(&self) {
        todo!()
    }
}
