use uuid::Uuid;
use wolf_engine_events::{
    mpsc::{self, MpscEventSender},
    EventReceiver,
};

#[non_exhaustive]
pub enum WindowEvent {
    CloseRequested { id: Uuid },
}

pub type WindowSystem = (EventQueue, Context);

pub struct WindowSettings {}

impl Default for WindowSettings {
    fn default() -> Self {
        Self {}
    }
}

pub struct Window {}

pub struct EventQueue {}

impl EventReceiver<()> for EventQueue {
    fn next_event(&mut self) -> Option<()> {
        None
    }
}

pub struct Context {}

impl Context {
    pub fn create_window(&self, settings: WindowSettings) -> Window {
        Window {}
    }
}

pub fn init() -> Result<WindowSystem, &'static str> {
    Ok((EventQueue {}, Context {}))
}

pub fn init_with_backend<T: WindowBackend + 'static>(backend: T) -> WindowSystem {
    let (event_sender, event_receiver) = mpsc::event_queue();
    let backend_adapter = backend.init(event_sender);
    let context = Context::new(backend_adapter);
    let event_queue = EventQueue::new(event_sender);
    (EventQueue {}, Context {})
}

pub trait WindowBackend {
    fn init(self, event_sender: MpscEventSender<WindowEvent>) -> Box<dyn WindowBackendAdapter>;
}

#[cfg_attr(test, mockall::automock)]
pub trait WindowBackendAdapter {
    fn pump_events(&mut self);
}

#[cfg(test)]
mod window_system_tests {
    use wolf_engine_events::EventSender;

    use super::*;

    struct TestWindowBackend {
        mock_window_system: MockWindowBackendAdapter,
        events: Vec<WindowEvent>,
    }

    impl TestWindowBackend {
        pub fn new(mock_window_system: MockWindowBackendAdapter) -> Self {
            Self {
                mock_window_system,
                events: Vec::new(),
            }
        }

        pub fn send_events(mut self, events: Vec<WindowEvent>) -> Self {
            self.events = events;
            self
        }
    }

    impl WindowBackend for TestWindowBackend {
        fn init(self, event_sender: MpscEventSender<WindowEvent>) -> Box<dyn WindowBackendAdapter> {
            for event in self.events {
                event_sender.send_event(event).unwrap();
            }
            Box::new(self.mock_window_system)
        }
    }

    #[test]
    pub fn should_support_custom_backends() {
        let test_adapter = MockWindowBackendAdapter::new();
        let (_event_queue, _context) =
            crate::init_with_backend(TestWindowBackend::new(test_adapter));
    }

    #[test]
    pub fn should_pump_backend_events_when_event_queue_is_cleared() {
        let mut test_adapter = MockWindowBackendAdapter::new();
        test_adapter.expect_pump_events().once().returning(|| ());

        let (mut event_queue, _context) = crate::init_with_backend(
            TestWindowBackend::new(test_adapter)
                .send_events(vec![WindowEvent::CloseRequested { id: Uuid::new_v4() }]),
        );

        assert!(event_queue.next_event().is_some());
        assert_eq!(event_queue.next_event(), None)
    }
}
