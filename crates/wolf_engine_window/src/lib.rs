use winit::{
    dpi::PhysicalSize,
    event::{Event as WinitEvent, WindowEvent as WinitWindowEvent},
    event_loop::{ControlFlow, EventLoop, EventLoopProxy},
    window::{Window as WinitWindow, WindowAttributes},
};

pub use winit;

/// Re-exports supported [`raw_window_handle`](crate::raw_window_handle::rwh_06) versions.
pub mod raw_window_handle;

/// Initialize the window system.
pub fn init() -> WindowContextBuilder {
    WindowContextBuilder::new()
}

/// An event generated by the window system.
#[derive(Copy, Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum WindowEvent {
    Resumed,
    RedrawRequested,
    Resized(u32, u32),
    Closed,
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
    pub fn build(self) -> WindowContext {
        let event_loop = EventLoop::with_user_event()
            .build()
            .expect("Failed to initialize the window system");
        self.build_with_event_loop(event_loop)
    }

    #[allow(deprecated)]
    fn build_with_event_loop(self, event_loop: EventLoop<BackendEvent>) -> WindowContext {
        WindowContext::new(event_loop, self.window_settings)
    }
}

/// Provides a simple window-system.
///
/// Create, and configure the Window Context with [`init()`].
pub struct WindowContext {
    event_loop: Option<WinitEventLoop>,
    event_loop_proxy: EventLoopProxy<BackendEvent>,
    window: Option<WinitWindow>,
    window_settings: WindowSettings,
}

impl WindowContext {
    fn new(event_loop: WinitEventLoop, window_settings: WindowSettings) -> Self {
        let event_loop_proxy = event_loop.create_proxy();
        Self {
            event_loop: Some(event_loop),
            event_loop_proxy,
            window: None,
            window_settings,
        }
    }
}

impl WindowContext {
    /// Get the current size of the window.
    ///
    /// # Panics
    ///
    /// - Will panic if the window has not been created yet.  This happens on
    /// [`WindowEvent::Resumed`].
    pub fn size(&self) -> (u32, u32) {
        let size = self.maybe_window().inner_size();
        (size.width, size.height)
    }

    /// Close the current window.
    ///
    /// The window system will stop after this.
    pub fn close(&self) {
        self.event_loop_proxy
            .send_event(BackendEvent::CloseRequested)
            .unwrap();
    }

    fn maybe_window(&self) -> &WinitWindow {
        self.window.as_ref().expect("Window not created yet")
    }

    /// Run the event-loop, passing events to the provided `event_handler`.
    #[allow(deprecated)]
    pub fn run<F: FnMut(WindowEvent, &WindowContext)>(mut self, mut event_handler: F) {
        let event_loop = std::mem::take(&mut self.event_loop).unwrap();
        let _ = event_loop.run(|event, event_loop| match event {
            WinitEvent::NewEvents(..) => {
                event_loop.set_control_flow(ControlFlow::Poll);
                if let Some(window) = &self.window {
                    window.request_redraw();
                }
            }
            WinitEvent::Resumed => {
                self.window = Some(
                    event_loop
                        .create_window(
                            WindowAttributes::default()
                                .with_title(&self.window_settings.title)
                                .with_inner_size(PhysicalSize::new(
                                    self.window_settings.size.0,
                                    self.window_settings.size.1,
                                ))
                                .with_resizable(self.window_settings.is_resizable)
                                .with_visible(self.window_settings.is_visible),
                        )
                        .expect("Window creation failed"),
                );
                (event_handler)(WindowEvent::Resumed, &self);
            }
            WinitEvent::WindowEvent {
                event: WinitWindowEvent::RedrawRequested,
                ..
            } => (event_handler)(WindowEvent::RedrawRequested, &self),
            WinitEvent::WindowEvent {
                event: WinitWindowEvent::Resized(size),
                ..
            } => (event_handler)(WindowEvent::Resized(size.width, size.height), &self),
            WinitEvent::WindowEvent {
                event: WinitWindowEvent::CloseRequested,
                ..
            }
            | WinitEvent::UserEvent(BackendEvent::CloseRequested) => {
                event_loop.exit();
            }
            WinitEvent::LoopExiting => {
                (event_handler)(WindowEvent::Closed, &self);
            }
            _ => (),
        });
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

impl rwh_06::HasWindowHandle for WindowContext {
    fn window_handle(&self) -> Result<rwh_06::WindowHandle<'_>, rwh_06::HandleError> {
        rwh_06::HasWindowHandle::window_handle(self.maybe_window())
    }
}

impl rwh_06::HasDisplayHandle for WindowContext {
    fn display_handle(&self) -> Result<rwh_06::DisplayHandle<'_>, rwh_06::HandleError> {
        rwh_06::HasDisplayHandle::display_handle(self.maybe_window())
    }
}

#[cfg(feature = "rwh_05")]
unsafe impl rwh_05::HasRawWindowHandle for WindowContext {
    fn raw_window_handle(&self) -> rwh_05::RawWindowHandle {
        rwh_05::HasRawWindowHandle::raw_window_handle(self.maybe_window())
    }
}

#[cfg(feature = "rwh_05")]
unsafe impl rwh_05::HasRawDisplayHandle for WindowContext {
    fn raw_display_handle(&self) -> rwh_05::RawDisplayHandle {
        rwh_05::HasRawDisplayHandle::raw_display_handle(self.maybe_window())
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
