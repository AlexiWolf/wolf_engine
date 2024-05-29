use wolf_engine_events::EventReceiver;

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
    (EventQueue {}, Context {})
}

pub trait WindowBackend {
}

#[cfg_attr(test, mockall::automock)]
pub trait WindowBackendAdapter {
}

#[cfg(test)]
mod window_system_tests {
    use super::*;

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

        let (mut event_queue, _context) =
            crate::init_with_backend(TestWindowBackend::new(test_adapter));

        assert_eq!(event_queue.next_event(), None)
    }
}
