use wgpu::rwh::{HasWindowHandle, WindowHandle};

pub fn init(graphics_settings: GraphicsSettings) -> GraphicsContextBuilder<'static> {
    GraphicsContextBuilder::<'static> {
        window_handle: None,
    }
}

#[derive(Default)]
pub struct GraphicsSettings {}

pub struct GraphicsContextBuilder<'window> {
    window_handle: Option<WindowHandle<'window>>,
}

impl<'window> GraphicsContextBuilder<'window> {
    pub fn with_window<W: HasWindowHandle>(mut self, window: &'window W) -> Self {
        self.window_handle = match window.window_handle() {
            Ok(window_handle) => Some(window_handle),
            Err(_) => None,
        };
        self
    }
    pub async fn build(self) -> Result<GraphicsContext, &'static str> {
        Ok(GraphicsContext {})
    }
}

pub struct GraphicsContext {}

#[cfg(test)]
mod graphics_init_tests {
    use super::*;

    #[pollster::test]
    async fn should_use_builder_pattern() {
        let graphics_settings = GraphicsSettings::default();
        let _graphics = crate::init(graphics_settings).build().await.unwrap();
    }

    #[test]
    fn should_default_window_to_none() {
        let graphics_builder = crate::init(GraphicsSettings::default());
        assert_eq!(graphics_builder.window_handle, None);
    }
}
