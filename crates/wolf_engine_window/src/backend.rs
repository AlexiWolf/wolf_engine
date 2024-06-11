use wolf_engine_events::mpsc::MpscEventSender;

use crate::{event::WindowEvent, settings::WindowSettings, Window};

pub trait WindowBackend {
    fn init(self, event_sender: MpscEventSender<WindowEvent>) -> Box<dyn WindowBackendAdapter>;
}

pub trait WindowBackendAdapter {
    fn pump_events(&self);
    fn create_window(&self, settings: WindowSettings) -> Window;
}
