use std::{
    collections::HashMap,
    sync::{Arc, RwLock, Weak},
};

use uuid::Uuid;
use wolf_engine_events::{
    dynamic::AnyEvent, mpsc::MpscEventSender, EventSender, ReceiverDroppedError,
};

use crate::{event::WindowContextEvent, Window, WindowSettings, WindowState};

#[derive(Clone)]
/// A link to the window system.
pub struct WindowContext {
    pub(crate) event_sender: MpscEventSender<AnyEvent>,
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
        let window = Window::new(self.clone(), window_state.clone());
        self.insert_window_state(&window_state);
        self.event_sender
            .send_event(Box::new(WindowContextEvent::WindowCreated(
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
            _ => (),
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

    pub(crate) fn remove_window_state(&self, uuid: Uuid) {
        self.window_states.write().unwrap().remove(&uuid);
    }

    pub fn insert_window_handle(
        &self,
        uuid: Uuid,
        window_handle: crate::raw_window_handle::WindowHandle,
    ) {
        self.with_window_state_mut(uuid, |window_state| {
            window_state.set_handle(window_handle);
        })
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
    use wolf_engine_events::{
        mpsc::{self, MpscEventReceiver},
        EventReceiver,
    };

    use crate::event::WindowContextEvent;

    use super::*;

    fn test_init() -> (
        MpscEventSender<AnyEvent>,
        MpscEventReceiver<AnyEvent>,
        WindowContext,
        WindowContextEventSender,
    ) {
        let (event_sender, event_receiver) = mpsc::event_queue();
        let (context, context_event_sender) = WindowContext::new(event_sender.clone());
        (event_sender, event_receiver, context, context_event_sender)
    }

    #[test]
    fn should_emit_window_created_events() {
        let (_, mut event_receiver, context, _context_event_sender) = test_init();
        let window_settings = WindowSettings::default().with_size((100, 100));
        let window = context.create_window(window_settings.clone());

        while let Some(event) = event_receiver.next_event() {
            if let Some(context_event) = event.downcast_ref::<WindowContextEvent>() {
                match context_event {
                    WindowContextEvent::WindowCreated(uuid, event_settings) => {
                        assert_eq!(*uuid, window.id(), "Event is for the wrong window uuid");
                        assert_eq!(
                            *event_settings, window_settings,
                            "The window settings do not match"
                        );
                        return;
                    }
                    _ => (),
                }
            }
        }

        panic!("The window created event was not emitted")
    }

    #[test]
    fn should_bookkeep_window_state() {
        fn window_count(context: &WindowContext) -> usize {
            context.window_states.read().unwrap().len()
        }

        let (_, _event_receiver, context, _context_event_sender) = test_init();

        assert_eq!(window_count(&context), 0);
        let a = context.create_window(WindowSettings::default());
        assert_eq!(window_count(&context), 1);
        let a_copy = a.clone();
        assert_eq!(
            window_count(&context),
            1,
            "Window clones should not change the window count"
        );

        let b = context.create_window(WindowSettings::default());
        assert_eq!(window_count(&context), 2);

        let c = context.create_window(WindowSettings::default());
        assert_eq!(window_count(&context), 3);

        drop(a);
        assert_eq!(
            window_count(&context),
            3,
            "Window should not be cleaned up until clone is dropped"
        );
        drop(a_copy);
        assert_eq!(
            window_count(&context),
            2,
            "Dropping last copy should clean up the window"
        );

        drop(b);
        assert_eq!(window_count(&context), 1);
        drop(c);
        assert_eq!(window_count(&context), 0);
    }

    #[test]
    fn should_emit_window_closed_events() {
        let (_, mut event_receiver, context, _context_event_sender) = test_init();
        let window = context.create_window(WindowSettings::default());
        let window_clone = window.clone();
        let window_id = window.id();

        drop(window);

        while let Some(event) = event_receiver.next_event() {
            if let Some(context_event) = event.downcast_ref::<WindowContextEvent>() {
                match context_event {
                    WindowContextEvent::WindowClosed(_uuid) => {
                        panic!("Window closed events should only be emitted when the last clone is dropped")
                    }
                    _ => (),
                }
            }
        }

        drop(window_clone);
        while let Some(event) = event_receiver.next_event() {
            if let Some(context_event) = event.downcast_ref::<WindowContextEvent>() {
                match context_event {
                    WindowContextEvent::WindowClosed(uuid) => {
                        assert_eq!(*uuid, window_id, "Event is for the wrong window uuid");
                        return;
                    }
                    _ => (),
                }
            }
        }

        panic!("The window close event was not emitted.");
    }

    #[test]
    fn should_resize_windows() {
        let (_, _event_receiver, context, context_event_sender) = test_init();
        let window = context.create_window(WindowSettings::default().with_size((100, 100)));

        context_event_sender
            .send_event(WindowContextEvent::WindowResized(window.id(), 800, 600))
            .unwrap();

        assert_eq!(window.size(), (800, 600), "The window was not resized");
    }

    #[test]
    fn should_emit_rename_events() {
        let (_, mut event_receiver, context, _context_event_sender) = test_init();
        let window = context.create_window(WindowSettings::default());

        window.set_title("I can haz rename?");

        while let Some(event) = event_receiver.next_event() {
            if let Some(context_event) = event.downcast_ref::<WindowContextEvent>() {
                match context_event {
                    WindowContextEvent::WindowRenameRequested(uuid, new_title) => {
                        assert_eq!(*uuid, window.id(), "Event is for the wrong window uuid");
                        assert_eq!(
                            new_title, "I can haz rename?",
                            "the new title is not correct"
                        );
                        return;
                    }
                    _ => (),
                }
            }
        }

        panic!("NO! Window cannot haz rename. :( \nThe rename event was not emitted.");
    }

    #[test]
    fn should_emit_redraw_requested_events() {
        let (_, mut event_receiver, context, _context_event_sender) = test_init();
        let window = context.create_window(WindowSettings::default());

        window.redraw();

        while let Some(event) = event_receiver.next_event() {
            if let Some(context_event) = event.downcast_ref::<WindowContextEvent>() {
                match context_event {
                    WindowContextEvent::WindowRedrawRequested(uuid) => {
                        assert_eq!(*uuid, window.id(), "Event is for the wrong window uuid");
                        return;
                    }
                    _ => (),
                }
            }
        }

        panic!("The redraw event was not emitted.");
    }

    #[test]
    fn should_emit_exited_events() {
        let (_, mut event_receiver, context, _context_event_sender) = test_init();

        context.exit();

        while let Some(event) = event_receiver.next_event() {
            if let Some(context_event) = event.downcast_ref::<WindowContextEvent>() {
                match context_event {
                    WindowContextEvent::Exited => {
                        return;
                    }
                    _ => (),
                }
            }
        }

        panic!("The exited event was not emitted.");
    }
}
