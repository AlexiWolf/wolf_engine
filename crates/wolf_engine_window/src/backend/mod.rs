pub(crate) mod winit;

use crate::{
    error::WindowError, raw_window_handle::HasRawWindowHandles, settings::WindowSettings, Window,
};

pub trait WindowTrait: HasRawWindowHandles + Send + Sync {
    fn title(&self) -> Result<String, WindowError>;
    fn size(&self) -> Result<(u32, u32), WindowError>;
    fn is_open(&self) -> bool;
    fn redraw(&self);
}

pub trait WindowBackend {
    fn pump_events(&self);
    fn create_window(&self, settings: WindowSettings) -> Window;
}
