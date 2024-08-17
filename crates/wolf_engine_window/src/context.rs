use anyhow::anyhow;
use uuid::Uuid;
use winit::{
    event_loop::ActiveEventLoop,
    window::{Fullscreen, WindowAttributes},
};

use crate::{
    error::{OsError, WindowError},
    FullscreenMode, Window, WindowIdMap, WindowSettings,
};

#[derive(Clone)]
/// A link to the window system.
pub struct WindowContext {}

impl WindowContext {
    pub(crate) fn new() -> Self {
        Self {}
    }

    /// Create a new [`Window`].
    pub fn create_window(&self, window_settings: WindowSettings) -> Uuid {
        todo!()
    }

    /// Stops the event loop.
    pub fn exit(&self) {
        todo!()
    }
}
