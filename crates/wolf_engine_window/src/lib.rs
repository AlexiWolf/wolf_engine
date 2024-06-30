use winit::{
    dpi::PhysicalSize,
    event::{Event as WinitEvent, WindowEvent as WinitWindowEvent},
    event_loop::{ControlFlow, EventLoop, EventLoopProxy},
    window::{Window as WinitWindow, WindowAttributes},
};

pub fn init() -> WindowContextBuilder {
    WindowContextBuilder::new()
}

#[non_exhaustive]
pub enum WindowEvent {
    Resume,
    Render,
    Closed,
}

type WinitEventLoop = winit::event_loop::EventLoop<BackendEvent>;

pub struct WindowContextBuilder {
    pub window_settings: WindowSettings,
}

impl WindowContextBuilder {
    pub fn new() -> Self {
        Self {
            window_settings: WindowSettings::default(),
        }
    }

    pub fn with_title(mut self, title: &str) -> Self {
        self.window_settings.title = title.to_string();
        self
    }

    pub fn with_size(mut self, size: (u32, u32)) -> Self {
        self.window_settings.size = size;
        self
    }

    pub fn with_resizable(mut self, is_resizable: bool) -> Self {
        self.window_settings.is_resizable = is_resizable;
        self
    }

    pub fn with_visible(mut self, is_visible: bool) -> Self {
        self.window_settings.is_visible = is_visible;
        self
    }

    pub fn build(self) -> WindowContext {
        let event_loop = EventLoop::with_user_event().build().unwrap();
        self.build_with_event_loop(event_loop)
    }

    #[allow(deprecated)]
    fn build_with_event_loop(self, event_loop: EventLoop<BackendEvent>) -> WindowContext {
        let winit_window = event_loop
            .create_window(
                WindowAttributes::default()
                    .with_title(self.window_settings.title)
                    .with_inner_size(PhysicalSize::new(
                        self.window_settings.size.0,
                        self.window_settings.size.1,
                    ))
                    .with_resizable(self.window_settings.is_resizable)
                    .with_visible(self.window_settings.is_visible),
            )
            .unwrap();
        WindowContext::new(event_loop, winit_window)
    }
}

pub struct WindowContext {
    event_loop: Option<WinitEventLoop>,
    event_loop_proxy: EventLoopProxy<BackendEvent>,
    window: WinitWindow,
}

impl WindowContext {
    fn new(event_loop: WinitEventLoop, window: WinitWindow) -> Self {
        let event_loop_proxy = event_loop.create_proxy();
        Self {
            event_loop: Some(event_loop),
            event_loop_proxy,
            window,
        }
    }
}

impl WindowContext {
    pub fn size(&self) -> (u32, u32) {
        let size = self.window.inner_size();
        (size.width, size.height)
    }

    pub fn close(&self) {
        self.event_loop_proxy
            .send_event(BackendEvent::CloseRequested)
            .unwrap();
    }

    #[allow(deprecated)]
    pub fn run<F: FnMut(WindowEvent, &WindowContext)>(mut self, mut event_handler: F) {
        let event_loop = std::mem::take(&mut self.event_loop).unwrap();
        let _ = event_loop.run(|event, event_loop| match event {
            WinitEvent::NewEvents(..) => {
                event_loop.set_control_flow(ControlFlow::Poll);
                self.window.request_redraw();
            }
            WinitEvent::Resumed => (event_handler)(WindowEvent::Resume, &self),
            WinitEvent::WindowEvent {
                event: WinitWindowEvent::RedrawRequested,
                ..
            } => (event_handler)(WindowEvent::Render, &self),
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
        rwh_06::HasWindowHandle::window_handle(&self.window)
    }
}

impl rwh_06::HasDisplayHandle for WindowContext {
    fn display_handle(&self) -> Result<rwh_06::DisplayHandle<'_>, rwh_06::HandleError> {
        rwh_06::HasDisplayHandle::display_handle(&self.window)
    }
}

#[cfg(feature = "rwh_05")]
unsafe impl rwh_05::HasRawWindowHandle for WindowContext {
    fn raw_window_handle(&self) -> rwh_05::RawWindowHandle {
        rwh_05::HasRawWindowHandle::raw_window_handle(&self.window)
    }
}

#[cfg(feature = "rwh_05")]
unsafe impl rwh_05::HasRawDisplayHandle for WindowContext {
    fn raw_display_handle(&self) -> rwh_05::RawDisplayHandle {
        rwh_05::HasRawDisplayHandle::raw_display_handle(&self.window)
    }
}

#[cfg(feature = "rwh_05")]
pub use rwh_05;

#[cfg(feature = "rwh_06")]
pub use rwh_06;

#[cfg(feature = "rwh_06")]
pub trait HasRwh6Handles: rwh_06::HasWindowHandle + rwh_06::HasDisplayHandle {}
#[cfg(feature = "rwh_06")]
impl<T> HasRwh6Handles for T where T: rwh_06::HasWindowHandle + rwh_06::HasDisplayHandle {}

#[cfg(not(feature = "rwh_06"))]
pub trait HasRwh6Handles {}
#[cfg(not(feature = "rwh_06"))]
impl<T> HasRwh6Handles for T {}

#[cfg(feature = "rwh_05")]
pub trait HasRwh5Handles: rwh_05::HasRawWindowHandle + rwh_05::HasRawDisplayHandle {}
#[cfg(feature = "rwh_05")]
impl<T> HasRwh5Handles for T where T: rwh_05::HasRawWindowHandle + rwh_05::HasRawDisplayHandle {}

#[cfg(not(feature = "rwh_05"))]
pub trait HasRwh5Handles {}
#[cfg(not(feature = "rwh_05"))]
impl<T> HasRwh5Handles for T {}

pub trait HasRawWindowHandles: HasRwh6Handles + HasRwh5Handles {}
impl<T> HasRawWindowHandles for T where T: HasRwh6Handles + HasRwh5Handles {}

#[cfg(test)]
mod window_init_tests {
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
    #[ntest::timeout(100)]
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
            WindowEvent::Resume => window.close(),
            WindowEvent::Closed => *&mut has_quit = true,
            _ => (),
        });

        assert!(
            has_quit,
            "The window system has not quit, or did not run properly."
        );
    }
}
