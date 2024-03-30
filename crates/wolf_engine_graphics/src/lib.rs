use wgpu::rwh::{HasWindowHandle, WindowHandle};

pub fn init() -> GraphicsContextBuilder {
    GraphicsContextBuilder {
        settings: GraphicsSettings::default(),
    }
}

#[derive(Default)]
pub struct GraphicsSettings {}

pub struct GraphicsContextBuilder {
    settings: GraphicsSettings,
}

impl GraphicsContextBuilder {
    pub async fn build(self) -> Result<GraphicsContext, &'static str> {
        Ok(GraphicsContext {})
    }
}

pub struct GraphicsContext {}

impl GraphicsContext {
    pub fn resize(&mut self, width: u32, height: u32) {}
}

#[cfg(test)]
mod graphics_init_tests {
    use super::*;

    #[pollster::test]
    async fn should_use_builder_pattern() {
        let _graphics = crate::init().build().await.unwrap();
    }
}
