use uuid::Uuid;

use crate::raw_window_handle::{HasRawWindowHandles, WindowHandle};

/// The fullscreen-mode for a Window.
#[derive(Clone, Eq, PartialEq, Debug)]
pub enum FullscreenMode {
    Borderless,
}

/// The settings used by the [`WindowContext`](crate::WindowContext) when creating the window.
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct WindowSettings {
    pub title: String,
    pub size: (u32, u32),
    pub is_resizable: bool,
    pub is_visible: bool,
    pub fullscreen_mode: Option<FullscreenMode>,
}

impl WindowSettings {
    /// Set the window's title.
    pub fn with_title(mut self, title: &str) -> Self {
        self.title = title.to_string();
        self
    }

    /// Set the window's size.
    pub fn with_size(mut self, size: (u32, u32)) -> Self {
        self.size = size;
        self
    }

    /// Set whether or not the window should be resizable.
    pub fn with_resizable(mut self, is_resizable: bool) -> Self {
        self.is_resizable = is_resizable;
        self
    }

    /// Set whether or not the window should be visible.
    pub fn with_visible(mut self, is_visible: bool) -> Self {
        self.is_visible = is_visible;
        self
    }

    pub fn with_fullscreen_mode(mut self, fullscreen_mode: FullscreenMode) -> Self {
        self.fullscreen_mode = Some(fullscreen_mode);
        self
    }
}

impl Default for WindowSettings {
    fn default() -> Self {
        Self {
            title: "Untitled".to_string(),
            size: (1280, 720),
            is_resizable: true,
            is_visible: true,
            fullscreen_mode: None,
        }
    }
}

/// A window.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Window {
    uuid: Uuid,
}

impl Window {
    pub(crate) fn new(uuid: Uuid, _window_settings: WindowSettings) -> Self {
        Self { uuid }
    }

    /// Get the uuid of the window.
    pub fn id(&self) -> Uuid {
        self.uuid
    }

    /// Get the current size of the window.
    pub fn size(&self) -> (u32, u32) {
        (0, 0)
    }

    /// Set the title of the window.
    pub fn set_title(&self, new_title: &str) {}

    /// Get the current fullscreen-mode, if the window is in full-screen.
    pub fn fullscreen_mode(&self) -> Option<FullscreenMode> {
        None
    }

    /// Set the fullscreen-mode.
    pub fn set_fullscreen_mode(&self, fullscreen_mode: Option<FullscreenMode>) {}

    /// Request a redraw of the window.
    pub fn redraw(&self) {}

    pub fn handle(&self) -> Option<WindowHandle> {
        None
    }
}
