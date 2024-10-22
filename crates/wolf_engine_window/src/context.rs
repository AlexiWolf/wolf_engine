use std::{
    collections::HashMap,
    sync::{Arc, RwLock, Weak},
};

use uuid::Uuid;
use wolf_engine_events::{
    dynamic::AnyEvent, mpsc::MpscEventSender, EventSender, ReceiverDroppedError,
};

use crate::{
    event::{WindowContextEvent, WindowFrontendEvent},
    Window, WindowSettings, WindowState,
};

#[derive(Clone)]
/// A link to the window system.
pub struct WindowContext {
    event_sender: MpscEventSender<AnyEvent>,
    window_states: Arc<RwLock<HashMap<Uuid, Weak<WindowState>>>>,
}

impl WindowContext {
    pub fn new(event_sender: MpscEventSender<AnyEvent>) -> (Self, WindowContextEventSender) {
        let context = Self {
            event_sender,
            window_states: Arc::new(RwLock::new(HashMap::new())),
        };
        let event_sender = WindowContextEventSender::new(context.clone());
        (context, event_sender)
    }

    /// Create a new [`Window`](crate::Window).
    pub fn create_window(&self, window_settings: WindowSettings) -> Window {
        let uuid = Uuid::new_v4();
        let window_state = Arc::new(WindowState::new(uuid, window_settings.clone()));
        let window = Window::new(self.event_sender.clone(), window_state.clone());
        self.insert_window_state(&window_state);
        self.event_sender
            .send_event(Box::new(WindowFrontendEvent::WindowCreated(
                uuid,
                window_settings,
            )))
            .unwrap();
        window
    }

    /// Stops the event loop.
    pub fn exit(&self) {
        todo!()
    }

    fn process_event(&self, event: WindowContextEvent) {
        match event {
            WindowContextEvent::WindowResized(uuid, width, height) => {
                self.with_window_state_mut(uuid, |window_state| {
                    window_state.resize(width, height);
                })
            }
        }
    }

    fn with_window_state_mut<F: FnOnce(Arc<WindowState>)>(&self, uuid: Uuid, function: F) {
        if let Some(weak) = self.window_states.write().unwrap().get_mut(&uuid) {
            if let Some(window_state) = Weak::upgrade(&weak) {
                function(window_state);
            }
        }
    }

    fn insert_window_state(&self, window_state: &Arc<WindowState>) {
        let uuid = window_state.uuid;
        self.window_states
            .write()
            .unwrap()
            .insert(uuid, Arc::downgrade(window_state));
    }
}

pub struct WindowContextEventSender {
    context: WindowContext,
}

impl WindowContextEventSender {
    fn new(context: WindowContext) -> Self {
        Self { context }
    }
}

impl EventSender<WindowContextEvent> for WindowContextEventSender {
    fn send_event(&self, event: WindowContextEvent) -> Result<(), ReceiverDroppedError> {
        self.context.process_event(event);
        Ok(())
    }
}

#[cfg(test)]
mod window_context_tests {
    use wolf_engine_events::mpsc;

    use crate::event::WindowContextEvent;

    use super::*;

    #[test]
    fn should_handle_incoming_events() {
        let (event_sender, _event_receiver) = mpsc::event_queue();
        let (context, context_event_sender) = WindowContext::new(event_sender.clone());

        let window = context.create_window(WindowSettings::default().with_size((100, 100)));

        context_event_sender
            .send_event(WindowContextEvent::WindowResized(window.id(), 800, 600))
            .unwrap();

        assert_eq!(window.size(), (800, 600), "The window was not resized");
    }
}
