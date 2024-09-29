use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use uuid::Uuid;
use wolf_engine_events::{dynamic::AnyEvent, mpsc::MpscEventSender, EventSender};

use crate::{event::BackendEvent, raw_window_handle::WindowHandle, Window, WindowSettings};

#[derive(Clone)]
/// A link to the window system.
pub struct WindowContext {
    event_sender: MpscEventSender<AnyEvent>,
    window_handles: Arc<RwLock<HashMap<Uuid, WindowHandle>>>,
}

impl WindowContext {
    pub(crate) fn new(event_sender: MpscEventSender<AnyEvent>) -> Self {
        Self {
            event_sender,
            window_handles: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Create a new [`Window`](crate::Window).
    pub fn create_window(&self, window_settings: WindowSettings) -> Window {
        let uuid = Uuid::new_v4();
        self.event_sender
            .send_event(Box::new(BackendEvent::CreateWindow(
                uuid,
                window_settings.clone(),
            )))
            .unwrap();
        Window::new(uuid, window_settings)
    }

    pub fn window_handle(&self, window: &Window) -> Option<WindowHandle> {
        match self.window_handles.read().unwrap().get(&window.id()) {
            Some(handle) => Some(handle.to_owned()),
            None => None,
        }
    }

    pub fn insert_window_handle(&self, uuid: Uuid, handle: WindowHandle) {
        self.window_handles.write().unwrap().insert(uuid, handle);
    }

    pub fn remove_window_handle(&self, uuid: Uuid) {
        self.window_handles.write().unwrap().remove(&uuid);
    }

    /// Stops the event loop.
    pub fn exit(&self) {
        self.event_sender
            .send_event(Box::new(BackendEvent::Exit))
            .unwrap();
    }
}

#[cfg(test)]
mod window_context_tests {
    use wolf_engine_events::{mpsc, EventReceiver};

    use super::*;

    #[test]
    fn should_sent_window_creation_events_to_backend() {
        let (event_sender, mut event_receiver) = mpsc::event_queue();
        let window_context = WindowContext::new(event_sender.clone());

        let window_settings = WindowSettings::default().with_title("Test Window");
        let window = window_context.create_window(window_settings.clone());

        while let Some(event) = event_receiver.next_event() {
            if let Some(backend_event) = event.downcast_ref::<BackendEvent>() {
                match backend_event {
                    BackendEvent::CreateWindow(uuid, settings) => {
                        assert_eq!(*uuid, window.id());
                        assert_eq!(*settings, window_settings);
                        return;
                    }
                    _ => (),
                }
            }
        }

        panic!("Event not emitted");
    }
}
