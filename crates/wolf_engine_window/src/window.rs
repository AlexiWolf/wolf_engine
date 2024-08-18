use std::{collections::HashMap, sync::Arc};

use uuid::Uuid;
use winit::{
    dpi::PhysicalSize,
    window::{Fullscreen, Window as WinitWindow, WindowAttributes, WindowId},
};
use wolf_engine_events::{mpsc::MpscEventSender, EventSender};

use crate::ContextEvent;

/// The fullscreen-mode for a Window.
#[derive(Clone, Eq, PartialEq, Debug)]
pub enum FullscreenMode {
    Borderless,
}

impl From<Fullscreen> for FullscreenMode {
    fn from(fullscreen: Fullscreen) -> Self {
        match fullscreen {
            Fullscreen::Borderless(_) => FullscreenMode::Borderless,
            Fullscreen::Exclusive(_) => panic!("Exclusive fullscreen is not yet supported"),
        }
    }
}

impl From<FullscreenMode> for Fullscreen {
    fn from(fullscreen_mode: FullscreenMode) -> Self {
        match fullscreen_mode {
            FullscreenMode::Borderless => Fullscreen::Borderless(None),
        }
    }
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

impl From<WindowSettings> for WindowAttributes {
    fn from(val: WindowSettings) -> Self {
        WindowAttributes::default()
            .with_title(val.title)
            .with_inner_size(PhysicalSize::new(val.size.0, val.size.1))
            .with_resizable(val.is_resizable)
            .with_visible(val.is_visible)
    }
}

/// A window.
#[derive(Clone)]
pub struct Window {
    uuid: Uuid,
    event_sender: MpscEventSender<ContextEvent>,
    inner: Arc<WinitWindow>,
}

impl PartialEq for Window {
    fn eq(&self, other: &Self) -> bool {
        self.uuid == other.uuid
    }
}

impl Eq for Window {}

impl Window {
    pub(crate) fn new(
        uuid: Uuid,
        event_sender: MpscEventSender<ContextEvent>,
        inner: WinitWindow,
    ) -> Self {
        Self {
            uuid,
            event_sender,
            inner: Arc::new(inner),
        }
    }

    /// Get the uuid of the window.
    pub fn id(&self) -> Uuid {
        self.uuid
    }

    /// Get the current size of the window.
    pub fn size(&self) -> (u32, u32) {
        let size = self.inner.inner_size();
        (size.width, size.height)
    }

    /// Set the title of the window.
    pub fn set_title(&self, new_title: &str) {
        self.inner.set_title(new_title);
    }

    /// Get the current fullscreen-mode, if the window is in full-screen.
    pub fn fullscreen_mode(&self) -> Option<FullscreenMode> {
        self.inner.fullscreen().map(|fullscreen| fullscreen.into())
    }

    /// Set the fullscreen-mode.
    pub fn set_fullscreen_mode(&self, fullscreen_mode: Option<FullscreenMode>) {
        let fullscreen = fullscreen_mode.map(|fullscreen_mode| fullscreen_mode.into());
        self.inner.set_fullscreen(fullscreen);
    }

    /// Request a redraw of the window.
    pub fn redraw(&self) {
        self.inner.request_redraw();
    }
}

impl rwh_06::HasWindowHandle for Window {
    fn window_handle(&self) -> Result<rwh_06::WindowHandle<'_>, rwh_06::HandleError> {
        rwh_06::HasWindowHandle::window_handle(&self.inner)
    }
}

impl rwh_06::HasDisplayHandle for Window {
    fn display_handle(&self) -> Result<rwh_06::DisplayHandle<'_>, rwh_06::HandleError> {
        rwh_06::HasDisplayHandle::display_handle(&self.inner)
    }
}

#[cfg(feature = "rwh_05")]
unsafe impl rwh_05::HasRawWindowHandle for Window {
    fn raw_window_handle(&self) -> rwh_05::RawWindowHandle {
        rwh_05::HasRawWindowHandle::raw_window_handle(&self.inner)
    }
}

#[cfg(feature = "rwh_05")]
unsafe impl rwh_05::HasRawDisplayHandle for Window {
    fn raw_display_handle(&self) -> rwh_05::RawDisplayHandle {
        rwh_05::HasRawDisplayHandle::raw_display_handle(&self.inner)
    }
}

impl Drop for Window {
    fn drop(&mut self) {
        let weak = Arc::downgrade(&self.inner);
        if weak.strong_count() == 1 {
            let _ = self
                .event_sender
                .send_event(ContextEvent::WindowDropped(self.uuid));
        }
    }
}

impl std::fmt::Debug for Window {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Window")
            .field("uuid", &self.uuid)
            .field("inner", &self.inner)
            .finish()
    }
}

#[derive(Clone, Debug)]
pub(crate) struct WindowIdMap {
    window_ids: HashMap<WindowId, Uuid>,
}

impl WindowIdMap {
    pub fn new() -> Self {
        Self {
            window_ids: HashMap::new(),
        }
    }

    pub fn insert(&mut self, window: &Window) {
        self.window_ids.insert(window.inner.id(), window.id());
    }

    pub fn remove(&mut self, uuid: Uuid) {
        if let Some(winit_id) = self.winit_id_of(uuid) {
            let _ = self.window_ids.remove(&winit_id);
        }
    }

    pub fn uuid_of(&self, winit_id: WindowId) -> Option<Uuid> {
        Some(self.window_ids.get(&winit_id)?.to_owned())
    }

    pub fn winit_id_of(&self, uuid: Uuid) -> Option<WindowId> {
        for (winit_id, stored_uuid) in self.window_ids.iter() {
            if uuid == *stored_uuid {
                return Some(*winit_id);
            }
        }
        None
    }
}
