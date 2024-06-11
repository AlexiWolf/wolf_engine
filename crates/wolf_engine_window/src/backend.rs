use wolf_engine_events::mpsc::MpscEventSender;

use crate::{
    error::WindowError, event::WindowEvent, raw_window_handle::HasRawWindowHandles,
    settings::WindowSettings, Window,
};

pub trait WindowTrait: HasRawWindowHandles + Send + Sync {
    fn title(&self) -> Result<String, WindowError>;
    fn size(&self) -> Result<(u32, u32), WindowError>;
    fn is_open(&self) -> bool;
    fn close(&self);
}

pub trait WindowBackend {
    fn init(self, event_sender: MpscEventSender<WindowEvent>) -> Box<dyn WindowBackendAdapter>;
}

pub trait WindowBackendAdapter {
    fn pump_events(&self);
    fn create_window(&self, settings: WindowSettings) -> Window;
}
