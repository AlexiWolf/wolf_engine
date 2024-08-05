//! Provides a simple, high-level window system.
//!
//! # Initializing the Window System
//!
//! Initialize the window system by calling the [`init()`] function.
//!
//! ```no_run
//! let window_context = wolf_engine_window::init().build().unwrap();
//! ```
//!
//! Once you've created the [`EventLoop`], you can call its [`run()`](EventLoop::run())
//! method to start the window system with the provided event-handling function.  
//! # Handling Events, and Creating Window
//!
//! ```no_run
//! # use wolf_engine_window::{
//! #   WindowSettings,
//! #   event::WindowEvent,
//! # };
//! #
//! # let window_context = wolf_engine_window::init().build().unwrap();
//! #
//! let mut window = None;
//! window_context.run(|event, context| match event {
//!     // The main-loop has started.
//!     // Do intial setup, like creating windows, render surfaces, ext. here.
//!     WindowEvent::Resumed => {
//!         println!("Hello, world!");
//!         window = Some(
//!             context.create_window(
//!                 WindowSettings::default()
//!                     .with_title("Example Window")
//!                     .with_size((800, 600)),
//!             )
//!         );
//!     }
//!     // A window should be redrawn.
//!     WindowEvent::RedrawRequested(_window_id) => {
//!         // Render code goes here!
//!     },
//!     // A window has / should close.
//!     WindowEvent::Closed(_window_id) => {
//!         context.exit(); // Stop the event loop.
//!     }
//!     // The main-loop will stop.
//!     WindowEvent::Exited => println!("Goodbye, World!"),
//!     _ => (),
//! });
//! ```
//!
//! # Drawing on the Window
//!
//! This crate doesn't provide its own rendering functions.  Instead, it implements
//! [`raw_window_handle`] traits in order for compatibility with external rendering libraries.

use std::sync::Arc;

use event::EventLoopBuilder;
use winit::{
    dpi::PhysicalSize,
    window::{Window as WinitWindow, WindowAttributes},
};

mod context;
pub use context::*;

pub mod event;
pub use uuid::Uuid;

/// Error-types used by the window system.
pub mod error;
/// Re-exports supported [`raw_window_handle`](crate::raw_window_handle::rwh_06) versions.
pub mod raw_window_handle;

/// Initialize the window system.
pub fn init() -> EventLoopBuilder {
    EventLoopBuilder::new()
}

/// The settings used by the [`WindowContext`] when creating the window.
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct WindowSettings {
    pub title: String,
    pub size: (u32, u32),
    pub is_resizable: bool,
    pub is_visible: bool,
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
}

impl Default for WindowSettings {
    fn default() -> Self {
        Self {
            title: "Untitled".to_string(),
            size: (1280, 720),
            is_resizable: true,
            is_visible: true,
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
#[derive(Clone, Debug)]
pub struct Window {
    uuid: Uuid,
    inner: Arc<WinitWindow>,
}

impl PartialEq for Window {
    fn eq(&self, other: &Self) -> bool {
        self.uuid == other.uuid
    }
}

impl Eq for Window {}

impl Window {
    pub(crate) fn new(inner: Arc<WinitWindow>) -> Self {
        Self {
            uuid: Uuid::new_v4(),
            inner,
        }
    }

    pub fn id(&self) -> Uuid {
        self.uuid
    }

    /// Get the current size of the window.
    pub fn size(&self) -> (u32, u32) {
        let size = self.inner.inner_size();
        (size.width, size.height)
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

#[cfg(test)]
mod window_init_tests {
    use super::*;

    #[cfg(target_os = "linux")]
    use winit::platform::x11::EventLoopBuilderExtX11;

    #[cfg(target_os = "windows")]
    use winit::platform::windows::EventLoopBuilderExtWindows;

    #[test]
    fn should_set_settings_values() {
        let window_settings = WindowSettings::default()
            .with_title("Custom Test Title")
            .with_size((123, 123))
            .with_resizable(false)
            .with_visible(false);

        assert_eq!(window_settings.title, "Custom Test Title");
        assert_eq!(window_settings.size, (123, 123));
        assert_eq!(window_settings.is_resizable, false);
        assert_eq!(window_settings.is_visible, false);
    }

    #[cfg(any(target_os = "linux", target_os = "windows"))]
    #[test]
    #[ntest::timeout(1000)]
    fn should_run_and_quit() {
        use crate::event::{WindowEvent, WinitEventLoop};

        let event_loop = WinitEventLoop::with_user_event()
            .with_any_thread(true)
            .build()
            .unwrap();
        let context = crate::init().build_with_event_loop(event_loop);

        let mut has_quit = false;

        context.run(|event, context| match event {
            WindowEvent::Resumed => {
                let _window = context
                    .create_window(WindowSettings::default().with_visible(false))
                    .expect("window creation succeeded");
                context.exit();
            }
            WindowEvent::Exited => *&mut has_quit = true,
            _ => (),
        });

        assert!(
            has_quit,
            "The window system has not quit, or did not run properly."
        );
    }
}
