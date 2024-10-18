use wolf_engine_events::{dynamic::AnyEvent, event_loop::EventLoop};
use wolf_engine_window::{backend::WindowSystem, error::WindowError};

pub fn init() -> Result<WinitBackend, WindowError> {
    todo!()
}

pub struct WinitBackend {}

impl WindowSystem for WinitBackend {
    fn context(&self) -> wolf_engine_window::WindowContext {
        todo!()
    }
}

impl EventLoop<AnyEvent> for WinitBackend {
    fn event_sender(&self) -> wolf_engine_events::mpsc::MpscEventSender<AnyEvent> {
        todo!()
    }

    fn run<F: FnMut(AnyEvent)>(self, event_handler: F) {
        todo!()
    }
}
