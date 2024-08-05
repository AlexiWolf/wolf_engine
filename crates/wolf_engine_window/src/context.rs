use std::{
    collections::HashMap,
    sync::{Arc, RwLock, Weak},
};

use anyhow::anyhow;
use uuid::Uuid;
use winit::{
    event_loop::ActiveEventLoop,
    window::{Window as WinitWindow, WindowId},
};

use crate::{
    error::{OsError, WindowError},
    Window, WindowSettings,
};

/// A link to the window system.
pub struct WindowContext<'event_loop> {
    event_loop: &'event_loop ActiveEventLoop,
    window_ids: Arc<RwLock<HashMap<WindowId, Uuid>>>,
    windows: Arc<RwLock<HashMap<Uuid, Weak<WinitWindow>>>>,
}

impl<'event_loop> WindowContext<'event_loop> {
    pub(crate) fn new(
        event_loop: &'event_loop ActiveEventLoop,
        window_ids: Arc<RwLock<HashMap<WindowId, Uuid>>>,
        windows: Arc<RwLock<HashMap<Uuid, Weak<WinitWindow>>>>,
    ) -> Self {
        Self {
            event_loop,
            window_ids,
            windows,
        }
    }

    /// Create a new [`Window`].
    pub fn create_window(&self, window_settings: WindowSettings) -> Result<Window, WindowError> {
        match self.event_loop.create_window(window_settings.into()) {
            Ok(winit_window) => {
                let window_id = winit_window.id();
                let window_arc = Arc::new(winit_window);
                let window_weak = Arc::downgrade(&window_arc);
                let window = Window::new(window_arc);

                self.window_ids
                    .write()
                    .expect("write-lock was acquired")
                    .insert(window_id, window.uuid);
                self.windows
                    .write()
                    .expect("write-lock was acquired")
                    .insert(window.id(), window_weak);
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
