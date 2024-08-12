use anyhow::anyhow;
use winit::{
    event_loop::ActiveEventLoop,
    window::{Fullscreen, WindowAttributes},
};

use crate::{
    error::{OsError, WindowError},
    FullscreenMode, Window, WindowIdMap, WindowSettings,
};

/// A link to the window system.
pub struct WindowContext<'event_loop> {
    event_loop: &'event_loop ActiveEventLoop,
    window_ids: WindowIdMap,
}

impl<'event_loop> WindowContext<'event_loop> {
    pub(crate) fn new(event_loop: &'event_loop ActiveEventLoop, window_ids: WindowIdMap) -> Self {
        Self {
            event_loop,
            window_ids,
        }
    }

    /// Create a new [`Window`].
    pub fn create_window(&self, window_settings: WindowSettings) -> Result<Window, WindowError> {
        let fullscreen_mode = window_settings.fullscreen_mode.clone();
        let mut window_attributes: WindowAttributes = window_settings.into();
        if let Some(fullscreen_mode) = fullscreen_mode {
            let monitor_handle = self.event_loop.primary_monitor();
            match fullscreen_mode {
                FullscreenMode::Borderless => {
                    window_attributes = window_attributes
                        .with_fullscreen(Some(Fullscreen::Borderless(monitor_handle)))
                }
            }
        }
        match self.event_loop.create_window(window_attributes) {
            Ok(winit_window) => {
                let window = Window::new(winit_window, self.window_ids.id_remover());
                self.window_ids.insert(&window);
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
