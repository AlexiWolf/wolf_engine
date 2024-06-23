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

    pub fn build(self) -> WindowContext {
        todo!()
    }
}

pub struct WindowContext {}

impl WindowContext {
    pub fn run<F: FnMut(WindowEvent, Window)>(mut self, event_handler: F) {}
}

pub struct WindowSettings {
    pub title: String,
    pub size: (u32, u32),
    pub is_resizable: bool,
}

impl Default for WindowSettings {
    fn default() -> Self {
        Self {
            title: "Untitled".to_string(),
            size: (1280, 720),
            is_resizable: true,
        }
    }
}

pub struct Window {}

#[cfg(test)]
mod window_init_tests {
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
    }
}
