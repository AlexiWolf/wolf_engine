use uuid::Uuid;
use wolf_engine_events::{dynamic::AnyEvent, mpsc::MpscEventSender, EventSender};

use crate::{Window, WindowSettings};

#[derive(Clone)]
/// A link to the window system.
pub struct WindowContext {
    event_sender: MpscEventSender<AnyEvent>,
}

impl WindowContext {
    pub fn new(event_sender: MpscEventSender<AnyEvent>) -> Self {
        todo!()
    }

    /// Create a new [`Window`](crate::Window).
    pub fn create_window(&self, window_settings: WindowSettings) -> Window {
        todo!()
    }

    /// Stops the event loop.
    pub fn exit(&self) {
        todo!()
    }

    pub(crate) fn event_sender(&self) -> MpscEventSender<AnyEvent> {
        todo!()
    }
}
