#[cfg(feature = "rwh_05")]
pub use rwh_05;

#[cfg(feature = "rwh_06")]
pub use rwh_06;

/// A type which has [rwh_06] handles.
#[cfg(feature = "rwh_06")]
pub trait HasRwh6Handles: rwh_06::HasWindowHandle + rwh_06::HasDisplayHandle {}
#[cfg(feature = "rwh_06")]
impl<T> HasRwh6Handles for T where T: rwh_06::HasWindowHandle + rwh_06::HasDisplayHandle {}

/// Placeholder for when the `rwh_06` feature is not enabled.
#[cfg(not(feature = "rwh_06"))]
pub trait HasRwh6Handles {}
#[cfg(not(feature = "rwh_06"))]
impl<T> HasRwh6Handles for T {}

/// A type which has [rwh_05] handles.
#[cfg(feature = "rwh_05")]
pub trait HasRwh5Handles: rwh_05::HasRawWindowHandle + rwh_05::HasRawDisplayHandle {}
#[cfg(feature = "rwh_05")]
impl<T> HasRwh5Handles for T where T: rwh_05::HasRawWindowHandle + rwh_05::HasRawDisplayHandle {}

/// Placeholder for when the `rwh_05` feature is not enabled.
#[cfg(not(feature = "rwh_05"))]
pub trait HasRwh5Handles {}
#[cfg(not(feature = "rwh_05"))]
impl<T> HasRwh5Handles for T {}

/// A type which has all currently-enabled [`raw_window_handle`](rwh_06) handles.
pub trait HasRawWindowHandles: HasRwh6Handles + HasRwh5Handles {}
impl<T> HasRawWindowHandles for T where T: HasRwh6Handles + HasRwh5Handles {}

pub struct WindowHandle<'a> {
    rwh_06_window: rwh_06::WindowHandle<'a>,
    rwh_06_display: rwh_06::DisplayHandle<'a>,

    #[cfg(feature = "rwh_05")]
    rwh_05_window: rwh_05::RawWindowHandle,
    #[cfg(feature = "rwh_05")]
    rwh_05_display: rwh_05::RawDisplayHandle,
}

impl rwh_06::HasWindowHandle for WindowHandle<'_> {
    fn window_handle(&self) -> Result<rwh_06::WindowHandle<'_>, rwh_06::HandleError> {
        Ok(self.rwh_06_window)
    }
}

impl rwh_06::HasDisplayHandle for WindowHandle<'_> {
    fn display_handle(&self) -> Result<rwh_06::DisplayHandle<'_>, rwh_06::HandleError> {
        Ok(self.rwh_06_display)
    }
}

#[cfg(feature = "rwh_05")]
unsafe impl rwh_05::HasRawWindowHandle for WindowHandle<'_> {
    fn raw_window_handle(&self) -> rwh_05::RawWindowHandle {
        self.rwh_05_window
    }
}

#[cfg(feature = "rwh_05")]
unsafe impl rwh_05::HasRawDisplayHandle for WindowHandle<'_> {
    fn raw_display_handle(&self) -> rwh_05::RawDisplayHandle {
        self.rwh_05_display
    }
}
