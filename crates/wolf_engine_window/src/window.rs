use uuid::Uuid;

use crate::{backend::WindowTrait, error::WindowError};

pub struct Window {
    id: Uuid,
    inner: Box<dyn WindowTrait>,
}

impl Window {
    pub fn new<T: WindowTrait + 'static>(inner: T) -> Self {
        Self {
            id: Uuid::new_v4(),
            inner: Box::new(inner),
        }
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn title(&self) -> Result<String, WindowError> {
        self.inner.title()
    }

    pub fn size(&self) -> Result<(u32, u32), WindowError> {
        self.inner.size()
    }

    pub fn is_open(&self) -> bool {
        self.inner.is_open()
    }

    pub fn close(&self) {
        self.inner.close()
    }
}

#[cfg(feature = "rwh_06")]
impl rwh_06::HasWindowHandle for Window {
    fn window_handle(&self) -> Result<rwh_06::WindowHandle<'_>, rwh_06::HandleError> {
        rwh_06::HasWindowHandle::window_handle(&self.inner)
    }
}

#[cfg(feature = "rwh_06")]
impl rwh_06::HasDisplayHandle for Window {
    fn display_handle(&self) -> Result<rwh_06::DisplayHandle<'_>, rwh_06::HandleError> {
        rwh_06::HasDisplayHandle::display_handle(&self.inner)
    }
}

#[cfg(feature = "rwh_05")]
unsafe impl rwh_05::HasRawWindowHandle for Window {
    fn raw_window_handle(&self) -> rwh_05::RawWindowHandle {
        rwh_05::HasRawWindowHandle::raw_window_handle(&*self.inner)
    }
}

#[cfg(feature = "rwh_05")]
unsafe impl rwh_05::HasRawDisplayHandle for Window {
    fn raw_display_handle(&self) -> rwh_05::RawDisplayHandle {
        rwh_05::HasRawDisplayHandle::raw_display_handle(&*self.inner)
    }
}
