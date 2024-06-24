use winit::event_loop::EventLoop;

pub fn init() -> WindowContextBuilder {
    WindowContextBuilder::new()
}

#[non_exhaustive]
pub enum WindowEvent {
    Resume,
    Render,
    Closed,
}

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
        let event_loop = EventLoop::new().unwrap();
        self.build_with_event_loop(event_loop)
    }

    fn build_with_event_loop(self, event_loop: EventLoop<()>) -> WindowContext {
        WindowContext::new(event_loop, self.window_settings)
    }
}

pub struct WindowContext {
    event_loop: EventLoop<()>,
    window_settings: WindowSettings,
}

impl WindowContext {
    fn new(event_loop: EventLoop<()>, window_settings: WindowSettings) -> Self {
        Self {
            event_loop,
            window_settings,
        }
    }
}

impl WindowContext {
    pub fn run<F: FnMut(WindowEvent, Window)>(mut self, event_handler: F) {}
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

pub struct Window {}

impl Window {
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
        let event_loop = EventLoop::builder().with_any_thread(true).build().unwrap();
        let context = crate::init()
            .with_visible(false)
            .build_with_event_loop(event_loop);

        context.run(|event, window| match event {
            WindowEvent::Resume => window.close(),
            _ => (),
        });
    }
}
