use wgpu::rwh::HasWindowHandle;

pub fn init() -> GraphicsContextBuilder {
    GraphicsContextBuilder {}
}

pub struct GraphicsContextBuilder {}

impl GraphicsContextBuilder {
    pub fn with_window<W: HasWindowHandle>(self, window: W) -> Self {
        self
    }
    pub async fn build(self) -> Result<GraphicsContext, &'static str> {
        Ok(GraphicsContext {})
    }
}

pub struct GraphicsContext {}

#[cfg(test)]
mod graphics_init_tests {
    #[pollster::test]
    async fn should_use_builder_pattern() {
        let graphics_settings = GraphicsSettings::default();
        let _graphics = crate::init(graphics_settings).build().await.unwrap();
    }
}
