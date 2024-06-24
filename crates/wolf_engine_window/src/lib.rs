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

    fn build_with_event_loop(self, event_loop: EventLoop<BackendEvent>) -> WindowContext {
        WindowContext::new(event_loop, self.window_settings)
    }
}

pub struct WindowContext {
    event_loop: WinitEventLoop,
    window_settings: WindowSettings,
}

impl WindowContext {
    fn new(event_loop: WinitEventLoop, window_settings: WindowSettings) -> Self {
        Self {
            event_loop,
            window_settings,
        }
    }
}

impl WindowContext {
    #[allow(deprecated)]
    pub fn run<F: FnMut(WindowEvent, Window)>(self, mut event_handler: F) {
        let (event_loop, window_settings) = (self.event_loop, self.window_settings);
        let event_loop_proxy = event_loop.create_proxy();
        let winit_window = event_loop
            .create_window(
                WindowAttributes::default()
                    .with_title(window_settings.title)
                    .with_inner_size(PhysicalSize::new(
                        window_settings.size.0,
                        window_settings.size.1,
                    ))
                    .with_resizable(window_settings.is_resizable)
                    .with_visible(window_settings.is_visible),
            )
            .unwrap();
        let window = Window::new(&winit_window, event_loop_proxy);

        let _ = event_loop.run(|event, event_loop| match event {
            WinitEvent::NewEvents(..) => {
                event_loop.set_control_flow(ControlFlow::Poll);
                winit_window.request_redraw();
            }
            WinitEvent::Resumed => (event_handler)(WindowEvent::Resume, window.clone()),
            WinitEvent::WindowEvent {
                event: WinitWindowEvent::RedrawRequested,
                ..
            } => (event_handler)(WindowEvent::Render, window.clone()),
            WinitEvent::WindowEvent {
                event: WinitWindowEvent::CloseRequested,
                ..
            }
            | WinitEvent::UserEvent(BackendEvent::CloseRequested) => {
                event_loop.exit();
            }
            WinitEvent::LoopExiting => {
                (event_handler)(WindowEvent::Closed, window.clone());
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

#[derive(Clone)]
pub struct Window<'a> {
    inner: &'a WinitWindow,
    event_loop_proxy: EventLoopProxy<BackendEvent>,
}

impl<'a> Window<'a> {
    fn new(inner: &'a WinitWindow, event_loop_proxy: EventLoopProxy<BackendEvent>) -> Self {
        Self {
            inner,
            event_loop_proxy,
        }
    }

    pub fn close(&self) {}
}

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
