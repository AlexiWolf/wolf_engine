pub(crate) mod winit;

use crate::{
    error::WindowError, raw_window_handle::HasRawWindowHandles, settings::WindowSettings, Window,
};

pub type WindowResult<T> = Result<T, WindowError>;

pub trait WindowTrait: HasRawWindowHandles + Send + Sync {
    fn title(&self) -> WindowResult<String>;
    fn size(&self) -> WindowResult<(u32, u32)>;
    fn redraw(&self) -> WindowResult<()>;
}

pub trait WindowBackend {
    fn pump_events(&self);
    fn create_window(&self, settings: WindowSettings) -> Window;
}
