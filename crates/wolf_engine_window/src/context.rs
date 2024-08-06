use std::sync::Arc;

use anyhow::anyhow;
use winit::event_loop::ActiveEventLoop;

use crate::{
    error::{OsError, WindowError},
    Window, WindowSettings, WindowStore,
};

/// A link to the window system.
pub struct WindowContext<'event_loop> {
    event_loop: &'event_loop ActiveEventLoop,
    window_store: Arc<WindowStore>,
}

impl<'event_loop> WindowContext<'event_loop> {
    pub(crate) fn new(
        event_loop: &'event_loop ActiveEventLoop,
        window_store: Arc<WindowStore>,
    ) -> Self {
        Self {
            event_loop,
            window_store,
        }
    }

    /// Create a new [`Window`].
    pub fn create_window(&self, window_settings: WindowSettings) -> Result<Window, WindowError> {
        match self.event_loop.create_window(window_settings.into()) {
            Ok(winit_window) => {
                let window = Window::new(Arc::new(winit_window));
                self.window_store.insert(&window);
                Ok(window)
            }
            Err(error) => Err(WindowError::OsError(OsError::from(anyhow!("{}", error)))),
        }
    }

    /// Stops the event loop.
    pub fn exit(&self) {
        self.event_loop.exit();
    }
}
