use wolf_engine_events::mpsc::{self, MpscEventSender};

use crate::{
    error::WindowError, event::WindowEvent, raw_window_handle::HasRawWindowHandles,
    settings::WindowSettings, Window, WindowContext, WindowEventQueue, WindowSystem,
};

pub trait WindowTrait: HasRawWindowHandles + Send + Sync {
    fn title(&self) -> Result<String, WindowError>;
    fn size(&self) -> Result<(u32, u32), WindowError>;
    fn is_open(&self) -> bool;
    fn close(&self);
}

pub fn init_with_backend<T: WindowBackend + 'static>(
    backend: T,
) -> Result<WindowSystem, &'static str> {
    let (event_sender, event_receiver) = mpsc::event_queue();
    let backend_adapter = backend.init(event_sender);
    let context = WindowContext::new(backend_adapter);
    let event_queue = WindowEventQueue::new(&context, event_receiver);
    Ok((event_queue, context))
}

pub trait WindowBackend {
    fn init(self, event_sender: MpscEventSender<WindowEvent>) -> Box<dyn WindowBackendAdapter>;
}

pub trait WindowBackendAdapter {
    fn pump_events(&self);
    fn create_window(&self, settings: WindowSettings) -> Window;
}
