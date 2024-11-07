use std::{fmt::Debug, sync::Arc};

#[cfg(feature = "rwh_05")]
pub use rwh_05;

#[cfg(feature = "rwh_06")]
pub use rwh_06;

use uuid::Uuid;

/// A type which has [rwh_06] handles.
#[cfg(feature = "rwh_06")]
pub trait HasRwh6Handles: rwh_06::HasWindowHandle + rwh_06::HasDisplayHandle {}
#[cfg(feature = "rwh_06")]
impl<T> HasRwh6Handles for T where T: rwh_06::HasWindowHandle + rwh_06::HasDisplayHandle {}

/// A placeholder for when the `rwh_06` feature is not enabled.
#[doc(hidden)]
#[cfg(not(feature = "rwh_06"))]
pub trait HasRwh6Handles {}
#[cfg(not(feature = "rwh_06"))]
impl<T> HasRwh6Handles for T {}

/// A type which has [rwh_05] handles.
#[cfg(feature = "rwh_05")]
pub trait HasRwh5Handles: rwh_05::HasRawWindowHandle + rwh_05::HasRawDisplayHandle {}
#[cfg(feature = "rwh_05")]
impl<T> HasRwh5Handles for T where T: rwh_05::HasRawWindowHandle + rwh_05::HasRawDisplayHandle {}

/// A placeholder for when the `rwh_05` feature is not enabled.
#[doc(hidden)]
#[cfg(not(feature = "rwh_05"))]
pub trait HasRwh5Handles {}
#[cfg(not(feature = "rwh_05"))]
impl<T> HasRwh5Handles for T {}

/// A type which has all currently-enabled `raw_window_handle` handles.
pub trait HasRawWindowHandles: HasRwh6Handles + HasRwh5Handles {}
impl<T> HasRawWindowHandles for T where T: HasRwh6Handles + HasRwh5Handles {}

/// Provides access to the platform-specific, raw window handle, and display handle of a
/// [`Window`](crate::Window).
#[derive(Clone)]
pub struct WindowHandle {
    uuid: Uuid,
    inner: Arc<dyn HasRawWindowHandles + Send + Sync>,
}

impl WindowHandle {
    pub fn new(inner: Arc<dyn HasRawWindowHandles + Send + Sync>) -> Self {
        Self {
            uuid: Uuid::new_v4(),
            inner,
        }
    }
}

impl PartialEq for WindowHandle {
    fn eq(&self, other: &Self) -> bool {
        self.uuid == other.uuid
    }
}

impl Debug for WindowHandle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut debug_struct = &mut f.debug_struct("WindowHandle");
        #[cfg(feature = "rwh_06")]
        {
            debug_struct = debug_struct
                .field(
                    "rwh_06_window_handle",
                    &rwh_06::HasWindowHandle::window_handle(&self.inner),
                )
                .field(
                    "rwh_06_display_handle",
                    &rwh_06::HasDisplayHandle::display_handle(&self.inner),
                );
        }

        #[cfg(feature = "rwh_05")]
        {
            debug_struct = debug_struct
                .field(
                    "rwh_05_window_handle",
                    &rwh_05::HasRawWindowHandle::raw_window_handle(&self.inner),
                )
                .field(
                    "rwh_05_display_handle",
                    &rwh_05::HasRawDisplayHandle::raw_display_handle(&self.inner),
                );
        }

        debug_struct.finish()
    }
}

#[cfg(feature = "rwh_06")]
impl rwh_06::HasWindowHandle for WindowHandle {
    fn window_handle(&self) -> Result<rwh_06::WindowHandle<'_>, rwh_06::HandleError> {
        rwh_06::HasWindowHandle::window_handle(self.inner.as_ref())
    }
}

#[cfg(feature = "rwh_06")]
impl rwh_06::HasDisplayHandle for WindowHandle {
    fn display_handle(&self) -> Result<rwh_06::DisplayHandle<'_>, rwh_06::HandleError> {
        rwh_06::HasDisplayHandle::display_handle(self.inner.as_ref())
    }
}

#[cfg(feature = "rwh_05")]
unsafe impl rwh_05::HasRawWindowHandle for WindowHandle {
    fn raw_window_handle(&self) -> rwh_05::RawWindowHandle {
        rwh_05::HasRawWindowHandle::raw_window_handle(self.inner.as_ref())
    }
}

#[cfg(feature = "rwh_05")]
unsafe impl rwh_05::HasRawDisplayHandle for WindowHandle {
    fn raw_display_handle(&self) -> rwh_05::RawDisplayHandle {
        rwh_05::HasRawDisplayHandle::raw_display_handle(self.inner.as_ref())
    }
}
