use wgpu::rwh::HasWindowHandle;

pub fn init() -> GraphicsContextBuilder {
    GraphicsContextBuilder {
        _settings: GraphicsSettings::default(),
    }
}

#[derive(Default)]
pub struct GraphicsSettings {}

pub struct GraphicsContextBuilder {
    _settings: GraphicsSettings,
}

impl GraphicsContextBuilder {
    pub async fn build(
        self,
        _window: Option<&dyn HasWindowHandle>,
    ) -> Result<GraphicsContext, &'static str> {
        Ok(GraphicsContext {})
    }
}

pub struct GraphicsContext {}

impl GraphicsContext {
    pub fn resize(&mut self, _width: u32, _height: u32) {}
}

#[cfg(test)]
mod graphics_init_tests {
    #[pollster::test]
    async fn should_use_builder_pattern() {
        let _graphics = crate::init().build(None).await.unwrap();
    }
}
