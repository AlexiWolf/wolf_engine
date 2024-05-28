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

pub fn init() -> WindowSystem {
    (EventQueue {}, Context {})
}

#[cfg(test)]
mod window_system_tests {
    pub fn should_create_window_system() {
        let test_backend = MockWindowBackend::new();
    }
}
