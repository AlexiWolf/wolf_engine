use std::sync::Arc;

use uuid::Uuid;
use wolf_engine_events::{mpsc::MpscEventReceiver, EventReceiver};

use crate::{backend::WindowBackendAdapter, context::WindowContext};

#[derive(Debug, PartialEq)]
#[non_exhaustive]
pub enum WindowEvent {
    CloseRequested { id: Uuid },
    Resized { id: Uuid, width: u32, height: u32 },
}

pub struct WindowEventQueue {
    backend_adapter: Arc<Box<dyn WindowBackendAdapter>>,
    event_receiver: MpscEventReceiver<WindowEvent>,
}

impl WindowEventQueue {
    pub(crate) fn new(
        context: &WindowContext,
        event_receiver: MpscEventReceiver<WindowEvent>,
    ) -> Self {
        let backend_adapter = context.backend_adapter();
        Self {
            backend_adapter,
            event_receiver,
        }
    }
}

impl EventReceiver<WindowEvent> for WindowEventQueue {
    fn next_event(&mut self) -> Option<WindowEvent> {
        let event = self.event_receiver.next_event();
        if event.is_none() {
            self.backend_adapter.pump_events();
        }
        event
    }
}
