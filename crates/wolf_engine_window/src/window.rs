use std::sync::{Arc, RwLock};

use uuid::Uuid;
use wolf_engine_events::EventSender;

use crate::{backend::event::WindowContextEvent, raw_window_handle::WindowHandle, WindowContext};

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
#[derive(Clone)]
pub struct Window {
    context: WindowContext,
    state: Arc<WindowState>,
}

impl Window {
    pub(crate) fn new(context: WindowContext, state: Arc<WindowState>) -> Self {
        Self { context, state }
    }

    /// Get the uuid of the window.
    pub fn id(&self) -> Uuid {
        self.state.uuid
    }

    /// Get the current size of the window.
    pub fn size(&self) -> (u32, u32) {
        self.state.size()
    }

    /// Set the title of the window.
    pub fn set_title(&self, new_title: &str) {
        self.context
            .event_sender
            .send_event(Box::new(WindowContextEvent::WindowRenameRequested(
                self.id(),
                new_title.into(),
            )))
            .unwrap();
    }

    /// Get the current fullscreen-mode, if the window is in full-screen.
    pub fn fullscreen_mode(&self) -> Option<FullscreenMode> {
        None
    }

    /// Set the fullscreen-mode.
    pub fn set_fullscreen_mode(&self, _fullscreen_mode: Option<FullscreenMode>) {}

    /// Request a redraw of the window.
    pub fn redraw(&self) {
        self.context
            .event_sender
            .send_event(Box::new(WindowContextEvent::WindowRedrawRequested(
                self.id(),
            )))
            .unwrap();
    }

    pub fn handle(&self) -> Option<WindowHandle> {
        self.state.handle()
    }
}

impl PartialEq for Window {
    fn eq(&self, other: &Self) -> bool {
        self.state.uuid == other.state.uuid
    }
}

impl Eq for Window {}

impl Drop for Window {
    fn drop(&mut self) {
        let weak = Arc::downgrade(&self.state);
        if weak.strong_count() == 1 {
            self.context.remove_window_state(self.id());
            let _ = self
                .context
                .event_sender
                .send_event(Box::new(WindowContextEvent::WindowClosed(self.id())));
        }
    }
}

pub(crate) struct WindowState {
    pub uuid: Uuid,
    pub settings: RwLock<WindowSettings>,
    pub handle: RwLock<Option<WindowHandle>>,
}

impl WindowState {
    pub fn new(uuid: Uuid, settings: WindowSettings) -> Self {
        Self {
            uuid,
            settings: RwLock::new(settings),
            handle: RwLock::new(None),
        }
    }

    pub fn size(&self) -> (u32, u32) {
        self.settings.read().unwrap().size
    }

    pub fn resize(&self, width: u32, height: u32) {
        self.settings.write().unwrap().size = (width, height);
    }

    pub fn handle(&self) -> Option<WindowHandle> {
        self.handle
            .read()
            .unwrap()
            .as_ref()
            .map(|handle| handle.to_owned())
    }

    pub fn set_handle(&self, handle: WindowHandle) {
        *self.handle.write().unwrap() = Some(handle);
    }
}
