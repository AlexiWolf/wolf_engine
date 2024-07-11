//! Provides a simple, high-level window system.
//!
//! # Initializing the Window System
//!
//! You can configure the window, and initialize the window system by calling
//! the [`init()`] function.
//!
//! ```no_run
//! let window_context = wolf_engine_window::init()
//!     .with_title("Test Window")
//!     .with_size((800, 600))
//!     .build()
//!     .unwrap();
//! ```
//!
//! # Running, and Handling Events
//!
//! Once you've created the [`WindowContext`], you can call its [`run()`](WindowContext::run())
//! method to start the window system with the provided event-handling function.
//!
//! ```no_run
//! # use wolf_engine_window::WindowEvent;
//! #
//! # let window_context = wolf_engine_window::init().build().unwrap();
//! #
//! window_context.run(|event, window_context| match event {
//!     // The window is created on Resumed, so this is where you can set up rendering.
//!     WindowEvent::Resumed => println!("Hello, world!"),
//!     // The window should be redrawn.  This is where you can put your main render code.
//!     // Redraws are automatically requsted each frame.
//!     WindowEvent::RedrawRequested => (),
//!     // The window has closed, and the loop will exit.
//!     WindowEvent::Closed => println!("Goodbye, World!"),
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
    marker::PhantomData,
    sync::{Arc, Weak},
};

use winit::{
    dpi::PhysicalSize,
    error::EventLoopError,
    event::{Event as WinitEvent, WindowEvent as WinitWindowEvent},
    event_loop::{ControlFlow, EventLoop, EventLoopProxy},
    window::{Window as WinitWindow, WindowAttributes},
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
#[derive(Copy, Clone, PartialEq)]
#[non_exhaustive]
pub enum WindowEvent {
    Resumed,
    RedrawRequested,
    Resized(u32, u32),
    Closed,
    Input(Input),
}

type WinitEventLoop = winit::event_loop::EventLoop<BackendEvent>;

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

    /// Set the window's title.
    pub fn with_title(mut self, title: &str) -> Self {
        self.window_settings.title = title.to_string();
        self
    }

    /// Set the window's size.
    pub fn with_size(mut self, size: (u32, u32)) -> Self {
        self.window_settings.size = size;
        self
    }

    /// Set whether or not the window should be resizable.
    pub fn with_resizable(mut self, is_resizable: bool) -> Self {
        self.window_settings.is_resizable = is_resizable;
        self
    }

    /// Set whether or not the window should be visible.
    pub fn with_visible(mut self, is_visible: bool) -> Self {
        self.window_settings.is_visible = is_visible;
        self
    }

    /// Initialize the window system.
    pub fn build(self) -> Result<WindowContext, EventLoopError> {
        match EventLoop::with_user_event().build() {
            Ok(event_loop) => Ok(self.build_with_event_loop(event_loop)),
            Err(error) => Err(error),
        }
    }

    #[allow(deprecated)]
    fn build_with_event_loop(self, event_loop: EventLoop<BackendEvent>) -> WindowContext {
        WindowContext::new(event_loop, self.window_settings)
    }
}

/// Type-states used by the [`WindowContext`].
pub mod context_state {

    /// Indicates the context has not run yet.
    pub struct Inactive;

    /// Indicates the context has run, and the window has been created.
    pub struct Active;
}

/// Provides a simple window-system.
///
/// Create, and configure the Window Context with [`init()`].
pub struct WindowContext<State = context_state::Inactive> {
    event_loop: Option<WinitEventLoop>,
    event_loop_proxy: EventLoopProxy<BackendEvent>,
    window: Option<Arc<WinitWindow>>,
    window_settings: WindowSettings,
    _state: PhantomData<State>,
}

impl WindowContext {
    fn new(event_loop: WinitEventLoop, window_settings: WindowSettings) -> Self {
        let event_loop_proxy = event_loop.create_proxy();
        Self {
            event_loop: Some(event_loop),
            event_loop_proxy,
            window: None,
            window_settings,
            _state: PhantomData,
        }
    }
}

impl WindowContext<context_state::Inactive> {
    fn create_running_context(self) -> WindowContext<context_state::Active> {
        WindowContext {
            event_loop: self.event_loop,
            event_loop_proxy: self.event_loop_proxy,
            window: self.window,
            window_settings: self.window_settings,
            _state: PhantomData,
        }
    }

    /// Run the event-loop, passing events to the provided `event_handler`.
    #[allow(deprecated)]
    pub fn run<F: FnMut(WindowEvent, &WindowContext<context_state::Active>)>(
        mut self,
        mut event_handler: F,
    ) {
        let event_loop = std::mem::take(&mut self.event_loop).unwrap();
        let mut context = self.create_running_context();
        let _ = event_loop.run(|event, event_loop| {
            if let Some(input) = event.to_input() {
                (event_handler)(WindowEvent::Input(input), &context);
            }
            match event {
                WinitEvent::NewEvents(..) => {
                    event_loop.set_control_flow(ControlFlow::Poll);
                    if let Some(window) = &context.window {
                        window.request_redraw();
                    }
                }
                WinitEvent::Resumed => {
                    context.window = Some(Arc::new(
                        event_loop
                            .create_window(
                                WindowAttributes::default()
                                    .with_title(&context.window_settings.title)
                                    .with_inner_size(PhysicalSize::new(
                                        context.window_settings.size.0,
                                        context.window_settings.size.1,
                                    ))
                                    .with_resizable(context.window_settings.is_resizable)
                                    .with_visible(context.window_settings.is_visible),
                            )
                            .expect("Window creation failed"),
                    ));
                    (event_handler)(WindowEvent::Resumed, &context);
                }
                WinitEvent::WindowEvent {
                    event: WinitWindowEvent::RedrawRequested,
                    ..
                } => (event_handler)(WindowEvent::RedrawRequested, &context),
                WinitEvent::WindowEvent {
                    event: WinitWindowEvent::Resized(size),
                    ..
                } => (event_handler)(WindowEvent::Resized(size.width, size.height), &context),
                WinitEvent::WindowEvent {
                    event: WinitWindowEvent::CloseRequested,
                    ..
                }
                | WinitEvent::UserEvent(BackendEvent::CloseRequested) => {
                    event_loop.exit();
                }
                WinitEvent::LoopExiting => {
                    (event_handler)(WindowEvent::Closed, &context);
                }
                _ => (),
            }
        });
    }
}

impl WindowContext<context_state::Active> {
    /// Close the current window.
    ///
    /// The window system will stop after this.
    pub fn close(&self) {
        self.event_loop_proxy
            .send_event(BackendEvent::CloseRequested)
            .unwrap();
    }

    pub fn window(&self) -> Window {
        let window = self.window.as_ref().expect("Window not created yet");
        Window::new(&window)
    }
}

/// The settings used by the [`WindowContext`] when creating the window.
#[derive(Clone, Eq, PartialEq)]
pub struct WindowSettings {
    pub title: String,
    pub size: (u32, u32),
    pub is_resizable: bool,
    pub is_visible: bool,
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

#[derive(Clone, Copy, Debug)]
enum BackendEvent {
    CloseRequested,
}

pub struct Window {
    inner: Weak<WinitWindow>,
}

impl Window {
    fn new(inner: &Arc<WinitWindow>) -> Self {
        Self {
            inner: Arc::downgrade(inner),
        }
    }

    /// Get the current size of the window.
    pub fn size(&self) -> (u32, u32) {
        let window = self.inner.upgrade().unwrap();
        let size = window.inner_size();
        (size.width, size.height)
    }
}

impl rwh_06::HasWindowHandle for Window {
    fn window_handle(&self) -> Result<rwh_06::WindowHandle<'_>, rwh_06::HandleError> {
        todo!()
    }
}

impl rwh_06::HasDisplayHandle for Window {
    fn display_handle(&self) -> Result<rwh_06::DisplayHandle<'_>, rwh_06::HandleError> {
        todo!()
    }
}

#[cfg(feature = "rwh_05")]
unsafe impl rwh_05::HasRawWindowHandle for Window {
    fn raw_window_handle(&self) -> rwh_05::RawWindowHandle {
        rwh_05::HasRawWindowHandle::raw_window_handle(&self.inner.upgrade().unwrap())
    }
}

#[cfg(feature = "rwh_05")]
unsafe impl rwh_05::HasRawDisplayHandle for Window {
    fn raw_display_handle(&self) -> rwh_05::RawDisplayHandle {
        rwh_05::HasRawDisplayHandle::raw_display_handle(&self.inner.upgrade().unwrap())
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
    fn should_set_builder_settings() {
        let context_builder = crate::init()
            .with_title("Custom Test Title")
            .with_size((123, 123))
            .with_resizable(false)
            .with_visible(false);

        let window_settings = context_builder.window_settings;

        assert_eq!(window_settings.title, "Custom Test Title");
        assert_eq!(window_settings.size, (123, 123));
        assert_eq!(window_settings.is_resizable, false);
        assert_eq!(window_settings.is_visible, false);
    }

    #[cfg(any(target_os = "linux", target_os = "windows"))]
    #[test]
    #[ntest::timeout(1000)]
    fn should_run_and_quit() {
        let event_loop = EventLoop::with_user_event()
            .with_any_thread(true)
            .build()
            .unwrap();
        let context = crate::init()
            .with_visible(false)
            .build_with_event_loop(event_loop);

        let mut has_quit = false;

        context.run(|event, window| match event {
            WindowEvent::Resumed => window.close(),
            WindowEvent::Closed => *&mut has_quit = true,
            _ => (),
        });

        assert!(
            has_quit,
            "The window system has not quit, or did not run properly."
        );
    }
}
