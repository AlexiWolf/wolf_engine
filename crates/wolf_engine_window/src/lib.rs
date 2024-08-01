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
//! Once you've created the [`WindowContext`], you can call its [`run()`](EventLoop::run())
//! method to start the window system with the provided event-handling function.  
//!
//! A [`Window`] can only be created after the window context has been activated by the
//! [`run()`](EventLoop::run()) method.
//! # Handling Events, and Creating Window
//!
//! ```no_run
//! # use wolf_engine_window::WindowEvent;
//! #
//! # let window_context = wolf_engine_window::init().build().unwrap();
//! #
//! window_context.run(|event, window_context| match event {
//!     // The application has started.
//!     // This is where you should do setup for your game, like creating the window, and setting
//!     // up the renderer.
//!     WindowEvent::Resumed => println!("Hello, world!"),
//!     // A window should be redrawn.
//!     WindowEvent::RedrawRequested(uuid) => (),
//!     // The window context has stopped, and the loop will exit.
//!     WindowEvent::Exited => println!("Goodbye, World!"),
//!     _ => (),
//! });
//! ```
//!
//! # Drawing on the Window
//!
//! Like [Winit](winit), this crate doesn't provide its own rendering functions.  Instead, it
//! integrates with the [`raw_window_handle`] crate in order to interoperate with external
//! rendering libraries.

use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use thiserror::Error;
use uuid::Uuid;
use winit::{
    dpi::PhysicalSize,
    error::EventLoopError,
    event::{Event as WinitEvent, WindowEvent as WinitWindowEvent},
    event_loop::{ActiveEventLoop, ControlFlow},
    window::{Window as WinitWindow, WindowAttributes, WindowId},
};

pub use winit;
use wolf_engine_input::{Input, ToInput};

/// Re-exports supported [`raw_window_handle`](crate::raw_window_handle::rwh_06) versions.
pub mod raw_window_handle;

/// Initialize the window system.
pub fn init() -> WindowContextBuilder {
    WindowContextBuilder::new()
}

/// An event generated by the window system.
#[derive(Clone, PartialEq)]
#[non_exhaustive]
pub enum WindowEvent {
    Resumed,
    WindowCreated(Result<Window, WindowCreationError>),
    RedrawRequested(Uuid),
    Resized(Uuid, u32, u32),
    Closed(Uuid),
    Input(Option<Uuid>, Input),
    Exited,
}

type WinitEventLoop = winit::event_loop::EventLoop<()>;

/// Provides a way to configure the [`WindowContext`].
///
/// Create a new builder by calling [`init()`].
pub struct WindowContextBuilder {
    pub window_settings: WindowSettings,
}

impl WindowContextBuilder {
    fn new() -> Self {
        Self {
            window_settings: WindowSettings::default(),
        }
    }

    /// Initialize the window system.
    pub fn build(self) -> Result<EventLoop, EventLoopError> {
        match WinitEventLoop::with_user_event().build() {
            Ok(event_loop) => Ok(self.build_with_event_loop(event_loop)),
            Err(error) => Err(error),
        }
    }

    #[allow(deprecated)]
    fn build_with_event_loop(self, event_loop: WinitEventLoop) -> EventLoop {
        EventLoop::new(event_loop)
    }
}

/// Provides a simple window-system.
///
/// Create, and configure the Window Context with [`init()`].
pub struct EventLoop {
    event_loop: WinitEventLoop,
}

impl EventLoop {
    fn new(event_loop: WinitEventLoop) -> Self {
        Self { event_loop }
    }
}

impl EventLoop {
    /// Run the event-loop, passing events to the provided `event_handler`.
    #[allow(deprecated)]
    pub fn run<F: FnMut(WindowEvent, &WindowContext)>(self, mut event_handler: F) {
        let window_ids: Arc<RwLock<HashMap<WindowId, Uuid>>> =
            Arc::new(RwLock::new(HashMap::new()));
        let _ = self.event_loop.run(|event, event_loop| {
            let context = WindowContext::new(event_loop, window_ids.clone());

            match event {
                WinitEvent::NewEvents(..) => {
                    event_loop.set_control_flow(ControlFlow::Poll);
                }
                WinitEvent::Resumed => (event_handler)(WindowEvent::Resumed, &context),
                WinitEvent::LoopExiting => (event_handler)(WindowEvent::Exited, &context),
                WinitEvent::DeviceEvent { event, .. } => {
                    if let Some(input) = event.to_input() {
                        (event_handler)(WindowEvent::Input(None, input), &context);
                    }
                }
                WinitEvent::WindowEvent {
                    window_id,
                    event: window_event,
                } => {
                    let uuid = window_ids
                        .read()
                        .expect("read-lock was acquired")
                        .get(&window_id)
                        .expect("the uuid was inserted on window creation")
                        .to_owned();
                    if let Some(input) = window_event.to_input() {
                        (event_handler)(WindowEvent::Input(Some(uuid), input), &context);
                    }
                    match window_event {
                        WinitWindowEvent::RedrawRequested => {
                            (event_handler)(WindowEvent::RedrawRequested(uuid), &context)
                        }
                        WinitWindowEvent::Resized(size) => (event_handler)(
                            WindowEvent::Resized(uuid, size.width, size.height),
                            &context,
                        ),
                        WinitWindowEvent::CloseRequested => {
                            (event_handler)(WindowEvent::Closed(uuid), &context)
                        }
                        _ => (),
                    }
                }
                _ => (),
            }
        });
    }
}

pub struct WindowContext<'event_loop> {
    event_loop: &'event_loop ActiveEventLoop,
    window_ids: Arc<RwLock<HashMap<WindowId, Uuid>>>,
}

impl<'event_loop> WindowContext<'event_loop> {
    fn new(
        event_loop: &'event_loop ActiveEventLoop,
        window_ids: Arc<RwLock<HashMap<WindowId, Uuid>>>,
    ) -> Self {
        Self {
            event_loop,
            window_ids,
        }
    }

    pub fn create_window(
        &self,
        window_settings: WindowSettings,
    ) -> Result<Window, WindowCreationError> {
        match self.event_loop.create_window(window_settings.into()) {
            Ok(winit_window) => {
                let window_id = winit_window.id();
                let window = Window::new(Arc::new(winit_window));
                self.window_ids
                    .write()
                    .expect("write-lock was acquired")
                    .insert(window_id, window.uuid);
                Ok(window)
            }
            Err(_) => Err(WindowCreationError::Unknown),
        }
    }

    pub fn exit(&self) {
        self.event_loop.exit();
    }
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

impl Into<WindowAttributes> for WindowSettings {
    fn into(self) -> WindowAttributes {
        WindowAttributes::default()
            .with_title(self.title)
            .with_inner_size(PhysicalSize::new(self.size.0, self.size.1))
            .with_resizable(self.is_resizable)
            .with_visible(self.is_visible)
    }
}

#[derive(Error, Copy, Clone, Debug, PartialEq, Eq)]
pub enum WindowCreationError {
    #[error("window creation failed for an unknown reason")]
    Unknown,
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
    fn new(inner: Arc<WinitWindow>) -> Self {
        Self {
            uuid: Uuid::new_v4(),
            inner,
        }
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
    #[cfg(any(target_os = "linux", target_os = "windows"))]
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
