use std::sync::{Arc, RwLock};

use uuid::Uuid;
use wolf_engine_events::{
    dynamic::{AnyEvent, AnyEventSender},
    mpsc::MpscEventSender,
};

use crate::{event::BackendEvent, raw_window_handle::WindowHandle};

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
    event_sender: MpscEventSender<AnyEvent>,
    state: Arc<WindowState>,
}

impl Window {
    pub(crate) fn new(
        uuid: Uuid,
        event_sender: MpscEventSender<AnyEvent>,
        settings: WindowSettings,
    ) -> Self {
        Self {
            event_sender,
            state: Arc::new(WindowState::new(uuid, settings)),
        }
    }

    pub(crate) fn state(&self) -> Arc<WindowState> {
        self.state.clone()
    }

    /// Get the uuid of the window.
    pub fn id(&self) -> Uuid {
        self.state.uuid
    }

    /// Get the current size of the window.
    pub fn size(&self) -> (u32, u32) {
        self.state.settings.read().unwrap().size
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
        match self.state.handle.read().unwrap().as_ref() {
            Some(handle) => Some(handle.to_owned()),
            None => None,
        }
    }

    pub fn set_handle(&self, handle: WindowHandle) {
        *self.state.handle.write().unwrap() = Some(handle);
    }
}

impl PartialEq for Window {
    fn eq(&self, other: &Self) -> bool {
        self.state == other.state
    }
}

impl std::fmt::Debug for Window {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Window")
            .field("state", &self.state)
            .finish()
    }
}

impl Drop for Window {
    fn drop(&mut self) {
        let weak_state = Arc::downgrade(&self.state);
        // Strong-count of 2 refs to account for the extra ref owned by the window context:
        //   Self's ref (1 ref) + WindowContext's ref (1 ref) = 2 refs
        // The ref owned by the WindowContext should not keep the window alive.
        if weak_state.strong_count() == 2 {
            self.event_sender
                .send_any_event(BackendEvent::WindowDropped(self.state.uuid))
                .unwrap();
        }
    }
}

pub(crate) struct WindowState {
    pub uuid: Uuid,
    pub settings: RwLock<WindowSettings>,
    pub handle: RwLock<Option<WindowHandle>>,
}

impl WindowState {
    fn new(uuid: Uuid, settings: WindowSettings) -> Self {
        Self {
            uuid,
            settings: RwLock::new(settings),
            handle: RwLock::new(None),
        }
    }
}

impl std::fmt::Debug for WindowState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WindowState")
            .field("uuid", &self.uuid)
            .field("settings", &self.settings)
            .field("has_handle", &self.handle.read().unwrap().is_some())
            .finish()
    }
}

impl PartialEq for WindowState {
    fn eq(&self, other: &Self) -> bool {
        self.uuid == other.uuid
    }
}
