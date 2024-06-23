pub fn init() -> WindowContextBuilder {
    todo!()
}

#[non_exhaustive]
pub enum WindowEvent {
    Resume,
    Render,
    Closed,
}

pub struct WindowContextBuilder {}

impl WindowContextBuilder {
    pub fn window_settings(&self) -> WindowSettings {
        todo!()
    }

    pub fn with_title(mut self, title: &str) -> Self {
        self
    }

    pub fn with_size(mut self, size: (u32, u32)) -> Self {
        self
    }

    pub fn resizable(mut self, is_resizable: bool) -> Self {
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

pub struct Window {}

#[cfg(test)]
mod window_init_tests {
    #[test]
    fn should_set_builder_settings() {
        let context_builder = crate::init()
            .with_title("Custom Test Title")
            .with_size((123, 123))
            .resizable(false);

        let window_settings = context_builder.window_settings();
    }
}
