use std::sync::Arc;

use uuid::Uuid;
use winit::WinitBackend;
use wolf_engine_events::{
    mpsc::{self, MpscEventReceiver, MpscEventSender},
    EventReceiver,
};

mod winit;

#[derive(Debug, PartialEq)]
#[non_exhaustive]
pub enum WindowEvent {
    CloseRequested { id: Uuid },
}

pub type WindowSystem = (EventQueue, Context);

pub struct WindowSettings {
    pub title: String,
    pub size: (u32, u32),
}

impl Default for WindowSettings {
    fn default() -> Self {
        Self {
            title: "untitled".to_string(),
            size: (1280, 720),
        }
    }
}

pub struct Window {
    id: Uuid,
    inner: Box<dyn WindowTrait>,
}

impl Window {
    pub fn new<T: WindowTrait + 'static>(inner: T) -> Self {
        Self {
            id: Uuid::new_v4(),
            inner: Box::new(inner),
        }
    }

    pub fn id(&self) -> Uuid {
        self.id
    }
}

pub trait WindowTrait {}

pub struct EventQueue {
    backend_adapter: Arc<Box<dyn WindowBackendAdapter>>,
    event_receiver: MpscEventReceiver<WindowEvent>,
}

impl EventQueue {
    fn new(context: &Context, event_receiver: MpscEventReceiver<WindowEvent>) -> Self {
        let backend_adapter = context.backend_adapter();
        Self {
            backend_adapter,
            event_receiver,
        }
    }
}

impl EventReceiver<WindowEvent> for EventQueue {
    fn next_event(&mut self) -> Option<WindowEvent> {
        let event = self.event_receiver.next_event();
        if event.is_none() {
            self.backend_adapter.pump_events();
        }
        event
    }
}

pub struct Context {
    backend_adapter: Arc<Box<dyn WindowBackendAdapter>>,
}

impl Context {
    fn new(backend_adapter: Box<dyn WindowBackendAdapter>) -> Self {
        Self {
            backend_adapter: Arc::new(backend_adapter),
        }
    }

    pub fn create_window(&self, settings: WindowSettings) -> Window {
        self.backend_adapter.create_window(settings)
    }

    fn backend_adapter(&self) -> Arc<Box<dyn WindowBackendAdapter>> {
        self.backend_adapter.clone()
    }
}

pub fn init() -> Result<WindowSystem, &'static str> {
    init_with_backend(WinitBackend)
}

pub fn init_with_backend<T: WindowBackend + 'static>(
    backend: T,
) -> Result<WindowSystem, &'static str> {
    let (event_sender, event_receiver) = mpsc::event_queue();
    let backend_adapter = backend.init(event_sender);
    let context = Context::new(backend_adapter);
    let event_queue = EventQueue::new(&context, event_receiver);
    Ok((event_queue, context))
}

pub trait WindowBackend {
    fn init(self, event_sender: MpscEventSender<WindowEvent>) -> Box<dyn WindowBackendAdapter>;
}

pub trait WindowBackendAdapter {
    fn pump_events(&self);
    fn create_window(&self, settings: WindowSettings) -> Window;
}

#[cfg(test)]
mod window_system_tests {
    use std::{collections::VecDeque, sync::RwLock};

    use wolf_engine_events::EventSender;

    use super::*;

    struct TestWindowBackend {
        adapter: TestWindowBackendAdapter,
    }

    impl TestWindowBackend {
        pub fn new(adapter: TestWindowBackendAdapter) -> Self {
            Self { adapter }
        }
    }

    impl WindowBackend for TestWindowBackend {
        fn init(self, event_sender: MpscEventSender<WindowEvent>) -> Box<dyn WindowBackendAdapter> {
            *self.adapter.event_sender.write().unwrap() = Some(event_sender);
            Box::new(self.adapter)
        }
    }

    #[derive(Clone)]
    pub struct TestWindowBackendAdapter {
        event_sender: Arc<RwLock<Option<MpscEventSender<WindowEvent>>>>,
        pub buffered_events: Arc<RwLock<VecDeque<WindowEvent>>>,
    }

    impl TestWindowBackendAdapter {
        fn new() -> Self {
            Self {
                event_sender: Arc::new(RwLock::new(None)),
                buffered_events: Arc::new(RwLock::new(VecDeque::new())),
            }
        }

        pub fn buffer_event(&self, event: WindowEvent) {
            self.buffered_events.write().unwrap().push_back(event);
        }
    }

    impl WindowBackendAdapter for TestWindowBackendAdapter {
        fn pump_events(&self) {
            while let Some(event) = self.buffered_events.write().unwrap().pop_front() {
                let guard = self.event_sender.read().unwrap();
                let event_queue = guard.as_ref().unwrap();
                event_queue.send_event(event).unwrap();
            }
        }

        fn create_window(&self, _settings: WindowSettings) -> Window {
            Window::new(TestWindow)
        }
    }

    struct TestWindow;

    impl WindowTrait for TestWindow {}

    #[test]
    pub fn should_pump_backend_events_when_event_queue_is_cleared() {
        let test_adapter = TestWindowBackendAdapter::new();
        let (mut event_queue, _context) =
            crate::init_with_backend(TestWindowBackend::new(test_adapter.clone())).unwrap();

        test_adapter.buffer_event(WindowEvent::CloseRequested { id: Uuid::new_v4() });

        assert!(
            event_queue.next_event().is_none(),
            "Expected `None`.  Events should be pumped only after a `None` is returned."
        );
        assert!(
            event_queue.next_event().is_some(),
            "Expected `Some`. Events should have been pumped by the previous `next_event()` call."
        );
    }
}
